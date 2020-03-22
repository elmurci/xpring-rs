use crate::i::prelude::*;
use fehler::throws;
use tokio::runtime::{Builder, Runtime};
use tonic::{metadata::MetadataValue, transport::Channel, Request};
use anyhow::bail;


#[derive(PartialEq, Debug)]
pub enum IlpPaymentStatus {
    FAILED,
    SUCCEEDED,
    UNKNOWN,
}

#[derive(PartialEq, Debug)]
pub struct IlpSendResponse {
    pub payment_status: IlpPaymentStatus,
    pub original_amount: u64,
    pub amount_delivered: u64,
    pub amount_sent: u64,
}

pub struct IlpClient {
    rt: Runtime,
    client: IlpOverHttpServiceClient<tonic::transport::Channel>,
    account_id: &'static str
}

impl IlpClient {
    #[throws(_)]
    pub(crate) fn connect(url: &'static str, account_id: &'static str, token: &'static str) -> Self {
        let mut rt = Builder::new()
            .basic_scheduler()
            .enable_all()
            .build()
            .unwrap();
            if token.is_empty() {
                bail!("token cannot be empty");
            }
        let channel = rt.block_on(Channel::from_static(url).connect())?;
        let token = MetadataValue::from_str(&format!("Bearer {}", token))?;
        let client =
            IlpOverHttpServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
                req.metadata_mut().insert("authorization", token.clone());
                Ok(req)
            });
        Self { rt, client, account_id }
    }

    // #[throws(_)]
    // pub(crate) fn get_balance(&mut self, jscontext: &mut JavaScript, x_address: &'static str) -> f32 {
    //     //
    // }

    #[throws(_)]
    pub(crate) fn send(
        &mut self,
        destination_payment_pointer: String,
        amount: u64,
        timeout_seconds: u64
    ) -> IlpSendResponse {
        let request = tonic::Request::new(SendPaymentRequest {
            destination_payment_pointer,
            amount,
            timeout_seconds,
            account_id: self.account_id.to_owned(),
        });

        match self.rt.block_on(self.client.send_money(request)) {
            Ok(result) => {
                let result = result.into_inner();
                let payment_status = if result.successful_payment {
                    IlpPaymentStatus::SUCCEEDED
                } else {
                    IlpPaymentStatus::FAILED
                };
                IlpSendResponse {
                    payment_status,
                    original_amount: result.original_amount,
                    amount_delivered: result.amount_delivered,
                    amount_sent: result.amount_sent,
                }
            },
            Err(_error) => {
                bail!("payment send failed");
            }
        } 
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const DEFAULT_SERVER_URL: &str = "http://hermes-grpc.ilpv4.dev";

    #[throws(_)]
    #[test]
    fn test_ilp_client_no_password() {
        match IlpClient::connect(DEFAULT_SERVER_URL, "test", "") {
            Ok(_result) => {
                assert!(false);
            }
            Err(error) => {
                assert_eq!(
                    "token cannot be empty",
                    error.to_string()
                );
            }
        }
    }

    #[throws(_)]
    #[test]
    fn test_ilp_client_ok() {
        match IlpClient::connect(DEFAULT_SERVER_URL, "test", "password") {
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
    fn test_ilp_client_invalid_url() {
        match IlpClient::connect("xrp", "test", "password") {
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
    fn test_send() {
        let mut client = IlpClient::connect(DEFAULT_SERVER_URL, "sdk_account1", "password")?;
        let original_amount = 12;
        match client.send(
            "$money.ilpv4.dev/sdk_account2".to_owned(),
            original_amount.clone(),
            10,
        ) {
            Ok(result) => {
                assert_eq!(result.original_amount, original_amount.clone());
                assert_eq!(result.amount_sent, original_amount.clone());
                assert_eq!(result.amount_delivered, original_amount.clone());
                assert_eq!(result.payment_status, IlpPaymentStatus::SUCCEEDED);
            }
            Err(_error) => {
                assert!(false);
            }
        }
    }

    #[throws(_)]
    #[test]
    fn test_send_wrong_account() {
        let mut client = IlpClient::connect(DEFAULT_SERVER_URL, "test.foo.bar", "password")?;
        let original_amount = 12;
        match client.send(
            "$money/baz".to_owned(),
            original_amount.clone(),
            10,
        ) {
            Ok(_result) => {
                assert!(false);
            }
            Err(_error) => {
                assert!(true);
            }
        }
    }
}
