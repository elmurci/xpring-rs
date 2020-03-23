use crate::address;
use crate::config;
use crate::javascript::JavaScript;
use crate::transaction;
use crate::transaction::{XAmount, XPayment, XRawTransactionStatus, XTransactionStatus};
use crate::wallet::XWallet;
use crate::x::prelude::*;
use anyhow::bail;
use fehler::throws;
use hex;
use std::str;
use std::thread;
use std::time::Duration;
use tokio::runtime::{Builder, Runtime};

type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(PartialEq, Debug)]
pub struct XrplReliableSendResponse {
    pub transaction_status: XTransactionStatus,
    pub transaction_hash: String,
    pub transaction_info: String,
}

pub(self) fn drops_to_decimal(drops: u64) -> f32 {
    drops as f32 / 1_000_000.
}

// The order of the fields in this struct is important. The runtime must be the first field and the
// client must be the last field so that when `BlockingClient` is dropped the client is dropped
// before the runtime. Not doing this will result in a deadlock when dropped.
pub struct XrplClient {
    rt: Runtime,
    client: XrpLedgerApiServiceClient<tonic::transport::Channel>,
}

impl XrplClient {
    #[throws(_)]
    pub(crate) fn connect<D>(url: D) -> Self
    where
        D: std::convert::TryInto<tonic::transport::Endpoint>,
        D::Error: Into<StdError>,
    {
        let mut rt = Builder::new()
            .basic_scheduler()
            .enable_all()
            .build()
            .unwrap();
        let client = rt.block_on(XrpLedgerApiServiceClient::connect(url))?;
        Self { rt, client }
    }

    //TODO Tests
    #[throws(_)]
    pub(self) fn get_fees(&mut self) -> GetFeeResponse {
        let request = tonic::Request::new(GetFeeRequest {});
        let response = self.rt.block_on(self.client.get_fee(request))?;
        response.into_inner()
    }

    #[throws(_)]
    #[allow(dead_code)]
    pub(self) fn get_base_fee(&mut self) -> u64 {
        let fees = self.get_fees()?;
        fees.fee.unwrap().base_fee.unwrap().drops
    }

    #[throws(_)]
    #[allow(dead_code)]
    pub(self) fn get_open_ledger_fee(&mut self) -> u64 {
        let fees = self.get_fees()?;
        fees.fee.unwrap().open_ledger_fee.unwrap().drops
    }

    #[throws(_)]
    pub(self) fn get_latest_validated_ledger_sequence(&mut self) -> u32 {
        let fees = self.get_fees()?;
        fees.ledger_current_index
    }

    #[throws(_)]
    pub(self) fn get_account_info(&mut self, address: &str) -> AccountRoot {
        let request = tonic::Request::new(GetAccountInfoRequest {
            account: Some(AccountAddress {
                address: address.to_owned(),
            }),
            signer_lists: false, //TODO
            strict: false,       //TODO
            ledger: None,        //TODO
            queue: false,        //TODO
        });
        let response = self.rt.block_on(self.client.get_account_info(request))?;
        response.into_inner().account_data.unwrap()
    }

    #[throws(_)]
    pub(self) fn get_account_sequence(
        &mut self,
        jscontext: &mut JavaScript,
        x_address: &'static str,
    ) -> u32 {
        let decoded_address = address::decode_x_address(jscontext, x_address)?;
        let account_info = self.get_account_info(&decoded_address.address)?;
        account_info.sequence.unwrap().value
    }

    #[throws(_)]
    pub(crate) fn get_balance(
        &mut self,
        jscontext: &mut JavaScript,
        x_address: &'static str,
    ) -> f32 {
        let decoded_address = address::decode_x_address(jscontext, x_address)?;
        let response = self.get_account_info(&decoded_address.address)?;
        if let currency_amount::Amount::XrpAmount(d) =
            response.balance.unwrap().value.unwrap().amount.unwrap()
        {
            drops_to_decimal(d.drops)
        } else {
            0.00
        }
    }

    #[throws(_)]
    pub(self) fn get_raw_transaction(
        &mut self,
        transaction_hash: Vec<u8>,
    ) -> GetTransactionResponse {
        let request = tonic::Request::new(GetTransactionRequest {
            hash: transaction_hash,
            binary: false,
            ledger_range: Some(LedgerRange {
                ledger_index_min: 1,
                ledger_index_max: 0, // all
            }),
        });
        let response = self.rt.block_on(self.client.get_transaction(request))?;
        response.into_inner()
    }

    #[throws(_)]
    pub(self) fn get_raw_transaction_status(
        &mut self,
        transaction_hash: &str,
    ) -> XRawTransactionStatus {
        let trx_hash_vec = hex::decode(transaction_hash)?;
        let response = self.get_raw_transaction(trx_hash_vec)?;
        let last_ledger_sequence =
            if let get_transaction_response::SerializedTransaction::Transaction(t) =
                response.serialized_transaction.unwrap()
            {
                t.last_ledger_sequence.unwrap().value
            } else {
                0
            };
        if let get_transaction_response::SerializedMeta::Meta(c) = response.serialized_meta.unwrap()
        {
            XRawTransactionStatus {
                transaction_result: c.transaction_result.unwrap(),
                last_ledger_sequence,
                validated: response.validated,
            }
        } else {
            bail!("Unknown raw transaction status");
        }
    }

    #[throws(_)]
    pub(crate) fn get_transaction_status(&mut self, transaction_hash: &str) -> XTransactionStatus {
        let transaction_status = self.get_raw_transaction_status(transaction_hash)?;
        transaction::from_raw_status(transaction_status)
    }

    #[throws(_)]
    pub(crate) fn send(
        &mut self,
        jscontext: &mut JavaScript,
        amount: f32,
        from_address: &'static str,
        to_address: &'static str,
        source_wallet: XWallet,
    ) -> XrplReliableSendResponse {
        let ledger_close_time_seconds = 4;
        let payment = XPayment {
            amount: XAmount::new(amount),
            from_address,
            to_address,
        };
        if !address::is_valid_x_address(jscontext, payment.to_address).unwrap()
            || !address::is_valid_x_address(jscontext, payment.from_address).unwrap()
        {
            bail!("Please use the X-Address format. See: https://xrpaddress.info.");
        }
        let account_sequence = self.get_account_sequence(jscontext, from_address)?;
        let latest_ledger = self.get_latest_validated_ledger_sequence()?;
        let last_validated_ledger_sequence = latest_ledger + config::MAX_LEDGER_VERSION_OFFSET;
        let transaction = transaction::build_payment_transaction(
            payment,
            12,
            account_sequence,
            last_validated_ledger_sequence,
            &source_wallet,
        )?;

        let signed_transaction =
            transaction::sign_transaction(jscontext, &transaction, &source_wallet)?;

        let request = tonic::Request::new(SubmitTransactionRequest {
            signed_transaction: hex::decode(signed_transaction.result).unwrap(),
            fail_hard: false,
        });
        let result = self.rt.block_on(self.client.submit_transaction(request))?;
        let response = result.into_inner();
        let result_transaction_status;
        let result_transaction_hash = hex::encode(&response.hash).to_uppercase();

        // The code tesSUCCESS is the only code that indicates a transaction succeeded
        // any other prefix will mean our transaction run into a problem
        let result_transaction_info = if !response.engine_result.unwrap().result.starts_with("tes")
        {
            result_transaction_status = XTransactionStatus::FAILED;
            response.engine_result_message
        } else {
            let mut latest_validated_ledger_sequence =
                self.get_latest_validated_ledger_sequence()?;
            let mut transaction_status =
                self.get_raw_transaction_status(&result_transaction_hash)?;
            while latest_validated_ledger_sequence <= last_validated_ledger_sequence
                && !transaction_status.validated
            {
                thread::sleep(Duration::from_secs(ledger_close_time_seconds));
                latest_validated_ledger_sequence = self.get_latest_validated_ledger_sequence()?;
                transaction_status = self.get_raw_transaction_status(&result_transaction_hash)?;
                if transaction_status.last_ledger_sequence == 0 {
                    bail!("The transaction did not have a last_ledger_sequence field so transaction status cannot be reliably determined.");
                }
            }
            result_transaction_status = transaction::from_raw_status(transaction_status);
            "".to_owned()
        };

        XrplReliableSendResponse {
            transaction_status: result_transaction_status,
            transaction_hash: result_transaction_hash,
            transaction_info: result_transaction_info,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const DEFAULT_SERVER_URL: &str = "http://test.xrp.xpring.io:50051";

    #[throws(_)]
    #[test]
    fn test_xrp_client_ok() {
        match XrplClient::connect(DEFAULT_SERVER_URL) {
            Ok(_result) => {
                assert!(true);
            }
            Err(_error) => {
                assert!(false);
            }
        }
    }

    #[throws(_)]
    #[test]
    fn test_xrp_client_invalid_url() {
        match XrplClient::connect("xrp") {
            Ok(_result) => {
                assert!(false);
            }
            Err(error) => {
                assert_eq!(
                    "transport error: error trying to connect: invalid URL, scheme is missing",
                    error.to_string()
                );
            }
        }
    }

    #[throws(_)]
    #[test]
    fn test_xpring_get_base_fee() {
        let mut client = XrplClient::connect(DEFAULT_SERVER_URL)?;
        let response = client.get_base_fee().unwrap();
        assert_eq!(response, 10);
    }

    #[throws(_)]
    #[test]
    fn test_xpring_get_balance() {
        let mut client = XrplClient::connect(DEFAULT_SERVER_URL)?;
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let response = client
            .get_balance(
                &mut jscontext,
                "TVr7v7JGN5suv7Zgdu9aL4PtCkwayZNYWvjSG23uMMWMvzZ",
            )
            .unwrap();
        assert_eq!(response, 1000.00);
    }

    #[throws(_)]
    #[test]
    fn test_xpring_raw_transaction_status() {
        let mut client = XrplClient::connect(DEFAULT_SERVER_URL)?;
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let w = XWallet::new(
            "0314ACE51F9B116BCF3C1E38A9BD92706AF4334165870139144E947B27BB0103E8".to_owned(),
            "009F56FC7B02354C428673EA14854616FED71888270C44911CBD87B84A5A59650F".to_owned(),
            false,
        );
        let payment = client.send(
            &mut jscontext,
            12.12,
            "T7jkn8zYC2NhPdcbVxkiEXZGy56YiEE4P7uXRgpy5j4Q6S1",
            "T7QqSicoC1nB4YRyzWzctWW7KjwiYUtDzVaLwFd4N7W1AUU",
            w,
        )?;
        thread::sleep(Duration::from_secs(4));
        let response = client.get_raw_transaction_status(
            &payment.transaction_hash,
        );
        assert_eq!(response.unwrap().transaction_result.result.starts_with("t"), true);
    }

    #[throws(_)]
    #[test]
    fn test_send() {
        let mut client = XrplClient::connect(DEFAULT_SERVER_URL)?;
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let w = XWallet::new(
            "0314ACE51F9B116BCF3C1E38A9BD92706AF4334165870139144E947B27BB0103E8".to_owned(),
            "009F56FC7B02354C428673EA14854616FED71888270C44911CBD87B84A5A59650F".to_owned(),
            false,
        );
        match client.send(
            &mut jscontext,
            12.12,
            "T7jkn8zYC2NhPdcbVxkiEXZGy56YiEE4P7uXRgpy5j4Q6S1",
            "T7QqSicoC1nB4YRyzWzctWW7KjwiYUtDzVaLwFd4N7W1AUU",
            w,
        ) {
            Ok(_result) => {
                assert!(true);
            }
            Err(_error) => {
                assert!(false);
            }
        }
    }
}
