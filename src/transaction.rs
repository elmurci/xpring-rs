use crate::javascript::{JavaScript, JsCall};
use crate::wallet::XWallet;
use crate::x::prelude::*;
use anyhow::bail;
use fehler::throws;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::str;

#[derive(PartialEq, Debug)]
pub enum XTransactionStatus {
    FAILED,
    PENDING,
    SUCCEEDED,
    UNKNOWN,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub(crate) struct XPayment {
    pub amount: XAmount,
    pub from_address: String,
    pub to_address: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub(crate) struct XAmount {
    pub drops: u64,
}

#[derive(PartialEq, Debug)]
pub(crate) struct XRawTransactionStatus {
    pub transaction_result: TransactionResult,
    pub last_ledger_sequence: u32,
    pub validated: bool,
}

impl XAmount {
    pub(crate) fn new(amount: f32) -> XAmount {
        let drops = amount * 1_000_000.;
        XAmount {
            drops: drops as u64,
        }
    }
}

#[derive(PartialEq, Deserialize, Debug)]
pub(crate) struct XSignedTransaction {
    pub result: String,
}

pub(crate) fn from_raw_status(raw_status: XRawTransactionStatus) -> XTransactionStatus {
    if !raw_status.validated {
        return XTransactionStatus::PENDING;
    }
    if raw_status.transaction_result.result.starts_with("tes") {
        XTransactionStatus::SUCCEEDED
    } else {
        XTransactionStatus::FAILED
    }
}

#[throws(_)]
pub(crate) fn serialize_transaction(transaction: &Transaction) -> Value {
    let trx_destination;
    let trx_xrp_amount;
    if let Some(TransactionData::Payment(c)) = &transaction.transaction_data {
        trx_destination = &c
            .destination
            .as_ref()
            .unwrap()
            .value
            .as_ref()
            .unwrap()
            .address;
        if let currency_amount::Amount::XrpAmount(d) = &c
            .amount
            .as_ref()
            .unwrap()
            .value
            .as_ref()
            .unwrap()
            .amount
            .as_ref()
            .unwrap()
        {
            trx_xrp_amount = d.drops;
        } else {
            bail!("Error parsing transaction amount");
        }
    } else {
        bail!("Error parsing transaction data");
    }
    let signing_public_key_hex =
        str::from_utf8(&transaction.signing_public_key.as_ref().unwrap().value);
    let trx = json!({
      "transaction": {
        "account": transaction.account.as_ref().unwrap().value.as_ref().unwrap().address,
        "fee": {
          "drops": transaction.fee.as_ref().unwrap().drops
        },
        "sequence": transaction.sequence.as_ref().unwrap().value,
        "payment": {
          "xrp_amount": {
            "drops": trx_xrp_amount
          },
          "destination": trx_destination
        },
        "signing_public_key_hex": signing_public_key_hex.unwrap(),
        "last_ledger_sequence": transaction.last_ledger_sequence.as_ref().unwrap().value
      }
    });
    trx
}

pub(crate) fn zero_vector<T>() -> Vec<T> {
    let zero_vec: Vec<T> = Vec::with_capacity(0 as usize);
    zero_vec
}

#[throws(_)]
pub(crate) fn sign_transaction(
    jscontext: &mut JavaScript,
    transaction: &Transaction,
    wallet: &XWallet,
) -> XSignedTransaction {
    let trx = serialize_transaction(transaction)?;
    let result = js!(jscontext
        .signer
        .signTransaction::<XSignedTransaction>(trx, wallet))?;
    result
}

#[throws(_)]
pub(crate) fn build_payment_transaction(
    payment: XPayment,
    fee: u64,
    trx_sequence: u32,
    last_ledger_sequence: u32,
    source_wallet: &XWallet,
) -> Transaction {
    let from = payment.from_address;
    let to = payment.to_address;
    let transaction_data = TransactionData::Payment(Payment {
        destination: Some(Destination {
            value: Some(AccountAddress {
                address: to.to_owned(),
            }),
        }),
        amount: Some(Amount {
            value: Some(CurrencyAmount {
                amount: Some(currency_amount::Amount::XrpAmount(XrpDropsAmount {
                    drops: payment.amount.drops,
                })),
            }),
        }),
        invoice_id: None,
        send_max: None,
        deliver_min: None,
        destination_tag: None, //TODO
        paths: zero_vector::<xPath>(),
    });

    let transaction = Transaction {
        account: Some(Account {
            value: Some(AccountAddress {
                address: from.to_owned(),
            }),
        }),
        fee: Some(XrpDropsAmount { drops: fee }),
        sequence: Some(Sequence {
            value: trx_sequence,
        }),
        signing_public_key: Some(SigningPublicKey {
            value: source_wallet.public_key.clone().as_bytes().to_vec(),
        }),
        last_ledger_sequence: Some(LastLedgerSequence {
            value: last_ledger_sequence,
        }),
        transaction_data: Some(transaction_data),
        account_transaction_id: None,
        flags: None,
        memos: zero_vector::<Memo>(),
        signers: zero_vector::<Signer>(),
        source_tag: None,
        transaction_signature: None,
    };

    transaction
}

#[cfg(test)]
mod tests {

    use super::*;
    use fehler::throws;

    #[throws(_)]
    #[test]
    fn test_from_raw_status_success() {
        let raw_status = XRawTransactionStatus {
            transaction_result: TransactionResult {
                result_type: 6,
                result: "tesSuccess".to_owned(),
            },
            last_ledger_sequence: 11239,
            validated: true,
        };
        assert_eq!(from_raw_status(raw_status), XTransactionStatus::SUCCEEDED);
    }

    #[throws(_)]
    #[test]
    fn test_from_raw_status_failed() {
        let raw_status = XRawTransactionStatus {
            transaction_result: TransactionResult {
                result_type: 6,
                result: "tefAlready".to_owned(),
            },
            last_ledger_sequence: 11239,
            validated: true,
        };
        assert_eq!(from_raw_status(raw_status), XTransactionStatus::FAILED);
    }

    #[throws(_)]
    #[test]
    fn test_from_raw_status_pending() {
        let raw_status = XRawTransactionStatus {
            transaction_result: TransactionResult {
                result_type: 6,
                result: "tesSuccess".to_owned(),
            },
            last_ledger_sequence: 11239,
            validated: false,
        };
        assert_eq!(from_raw_status(raw_status), XTransactionStatus::PENDING);
    }

    #[throws(_)]
    #[test]
    fn test_build_payment_transaction() {
        let w = XWallet::new(
            "031D68BC1A142E6766B2BDFB006CCFE135EF2E0E2E94ABB5CF5C9AB6104776FBAE".to_owned(),
            "0090802A50AA84EFB6CDB225F17C27616EA94048C179142FECF03F4712A07EA7A4".to_owned(),
            false,
        );
        let p = XPayment {
            amount: XAmount::new(12.12),
            from_address: "XVwDxLQ4SN9pEBQagTNHwqpFkPgGppXqrMoTmUcSKdCtcK5".to_owned(),
            to_address: "XVfC9CTCJh6GN2x8bnrw3LtdbqiVCUFyQVMzRrMGUZpokKH".to_owned(),
        };
        let r = build_payment_transaction(p, 12, 10, 139019301, &w).unwrap();
        assert_eq!(
            r.account.unwrap().value.unwrap().address,
            "XVwDxLQ4SN9pEBQagTNHwqpFkPgGppXqrMoTmUcSKdCtcK5".to_owned()
        );
        assert_eq!(
            r.signing_public_key.unwrap().value,
            "031D68BC1A142E6766B2BDFB006CCFE135EF2E0E2E94ABB5CF5C9AB6104776FBAE"
                .as_bytes()
                .to_vec()
        );
        assert_eq!(r.fee.unwrap().drops, 12);
    }

    #[throws(_)]
    #[test]
    fn test_sign_transaction_no_destination_tag() {
        let w = XWallet::new(
            "0314ACE51F9B116BCF3C1E38A9BD92706AF4334165870139144E947B27BB0103E8".to_owned(),
            "009F56FC7B02354C428673EA14854616FED71888270C44911CBD87B84A5A59650F".to_owned(),
            false,
        );
        let p = XPayment {
            amount: XAmount::new(0.000010),
            from_address: "T7jkn8zYC2NhPdcbVxkiEXZGy56YiEE4P7uXRgpy5j4Q6S1".to_owned(),
            to_address: "T7QqSicoC1nB4YRyzWzctWW7KjwiYUo9ZAXPrwRoKJ7FudP".to_owned(),
        };
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let t = build_payment_transaction(p, 10, 11, 5141976, &w).unwrap();
        let signed_transaction = sign_transaction(&mut jscontext, &t, &w)?;
        assert_eq!(signed_transaction.result, "120000240000000B201B004E75D861400000000000000A68400000000000000A73210314ACE51F9B116BCF3C1E38A9BD92706AF4334165870139144E947B27BB0103E87446304402201347C052098361A5F32155A42A2BB43ADF6A29B93B9512705E6B1960FF3016900220702A450E1F8A674BBEC32CF8926ED5E35F88A31AD9B5B7FA8940C667EC963AE881144594AF4CCC84B8E0AE58E6465F0BE056F0F70392831405EEB009A9DAE7DFBBB13523EA5CAB0B9B4B2E99".to_owned());
    }

    #[throws(_)]
    #[test]
    fn test_sign_transaction_destination_tag() {
        let w = XWallet::new(
            "0314ACE51F9B116BCF3C1E38A9BD92706AF4334165870139144E947B27BB0103E8".to_owned(),
            "009F56FC7B02354C428673EA14854616FED71888270C44911CBD87B84A5A59650F".to_owned(),
            false,
        );
        let p = XPayment {
            amount: XAmount::new(0.000010),
            from_address: "T7jkn8zYC2NhPdcbVxkiEXZGy56YiEE4P7uXRgpy5j4Q6S1".to_owned(),
            to_address: "T7QqSicoC1nB4YRyzWzctWW7KjwiYUtDzVaLwFd4N7W1AUU".to_owned(),
        };
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let mut jscontext = JavaScript::new(format!("{}/xpring.js", out_dir))?;
        let t = build_payment_transaction(p, 10, 11, 5141976, &w).unwrap();
        let signed_transaction = sign_transaction(&mut jscontext, &t, &w)?;
        assert_eq!(signed_transaction.result, "120000240000000B2E0000000D201B004E75D861400000000000000A68400000000000000A73210314ACE51F9B116BCF3C1E38A9BD92706AF4334165870139144E947B27BB0103E87447304502210093C4587DA120D86CFA76E8D87DFA35CE587013AAC7680D334C0CEC11741D7E9A022068CB5EED93624D3114ACF7DAB6C47EFEA1474E9BD0FE49E0B0D4FCE5A233586F81144594AF4CCC84B8E0AE58E6465F0BE056F0F70392831405EEB009A9DAE7DFBBB13523EA5CAB0B9B4B2E99".to_owned());
    }
}
