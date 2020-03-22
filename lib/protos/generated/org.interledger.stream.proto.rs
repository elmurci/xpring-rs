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
#[doc = r" Generated server implementations."]
pub mod ilp_over_http_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with IlpOverHttpServiceServer."]
    #[async_trait]
    pub trait IlpOverHttpService: Send + Sync + 'static {
        #[doc = " Send a payment through Hermes to a given payment pointer"]
        async fn send_money(
            &self,
            request: tonic::Request<super::SendPaymentRequest>,
        ) -> Result<tonic::Response<super::SendPaymentResponse>, tonic::Status>;
    }
    #[doc = " RPCs available to interact with Hermes."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct IlpOverHttpServiceServer<T: IlpOverHttpService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: IlpOverHttpService> IlpOverHttpServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T: IlpOverHttpService> Service<http::Request<HyperBody>> for IlpOverHttpServiceServer<T> {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<HyperBody>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/org.interledger.stream.proto.IlpOverHttpService/SendMoney" => {
                    struct SendMoneySvc<T: IlpOverHttpService>(pub Arc<T>);
                    impl<T: IlpOverHttpService>
                        tonic::server::UnaryService<super::SendPaymentRequest> for SendMoneySvc<T>
                    {
                        type Response = super::SendPaymentResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendPaymentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.send_money(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SendMoneySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: IlpOverHttpService> Clone for IlpOverHttpServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: IlpOverHttpService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: IlpOverHttpService> tonic::transport::NamedService for IlpOverHttpServiceServer<T> {
        const NAME: &'static str = "org.interledger.stream.proto.IlpOverHttpService";
    }
}
