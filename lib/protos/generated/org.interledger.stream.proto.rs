/// Defines the fields required to initiate a STREAM payment.
/// Next field: 4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendPaymentRequest {
    /// The destination PP to pay.
    #[prost(string, tag = "1")]
    pub destination_payment_pointer: std::string::String,
    /// The amount of this payment in the units and scale of the account.
    #[prost(uint64, tag = "2")]
    pub amount: u64,
    /// TODO: Should the client be able to specify this?
    /// The number of seconds to wait for this payment to complete.
    #[prost(uint64, tag = "3")]
    pub timeout_seconds: u64,
    /// Account Id of the sender
    #[prost(string, tag = "4")]
    pub account_id: std::string::String,
}
/// Defines the fields that are returned after a SendPayment RPC has completed.
/// Next field: 4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendPaymentResponse {
    /// The original amount that was requested to be sent.
    #[prost(uint64, tag = "1")]
    pub original_amount: u64,
    /// The actual amount, in the receivers units, that was delivered to the receiver
    #[prost(uint64, tag = "2")]
    pub amount_delivered: u64,
    /// The actual amount, in the senders units, that was sent to the receiver
    #[prost(uint64, tag = "3")]
    pub amount_sent: u64,
    /// Indicates if the payment was completed successfully.
    #[prost(bool, tag = "4")]
    pub successful_payment: bool,
}
#[doc = r" Generated client implementations."]
pub mod ilp_over_http_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " RPCs available to interact with Hermes."]
    pub struct IlpOverHttpServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IlpOverHttpServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> IlpOverHttpServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Send a payment through Hermes to a given payment pointer"]
        pub async fn send_money(
            &mut self,
            request: impl tonic::IntoRequest<super::SendPaymentRequest>,
        ) -> Result<tonic::Response<super::SendPaymentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/org.interledger.stream.proto.IlpOverHttpService/SendMoney",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for IlpOverHttpServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
}
/// Return fields of a balance request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBalanceResponse {
    #[prost(string, tag = "1")]
    pub account_id: std::string::String,
    #[prost(string, tag = "2")]
    pub asset_code: std::string::String,
    #[prost(int32, tag = "3")]
    pub asset_scale: i32,
    #[prost(int64, tag = "4")]
    pub net_balance: i64,
    #[prost(int64, tag = "5")]
    pub prepaid_amount: i64,
    #[prost(int64, tag = "6")]
    pub clearing_balance: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBalanceRequest {
    #[prost(string, tag = "1")]
    pub account_id: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod balance_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " RPCs available to interact with Hermes."]
    pub struct BalanceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BalanceServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BalanceServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Get the balance of a certain account on a connector"]
        pub async fn get_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBalanceRequest>,
        ) -> Result<tonic::Response<super::GetBalanceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/org.interledger.stream.proto.BalanceService/GetBalance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for BalanceServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
}
