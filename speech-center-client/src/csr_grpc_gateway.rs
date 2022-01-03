#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecognitionRequest {
    #[prost(oneof = "recognition_request::RequestUnion", tags = "1, 2")]
    pub request_union: ::core::option::Option<recognition_request::RequestUnion>,
}
/// Nested message and enum types in `RecognitionRequest`.
pub mod recognition_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestUnion {
        #[prost(message, tag = "1")]
        Init(super::RecognitionInit),
        #[prost(bytes, tag = "2")]
        Audio(::prost::alloc::vec::Vec<u8>),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecognitionInit {
    #[prost(message, optional, tag = "1")]
    pub parameters: ::core::option::Option<RecognitionParameters>,
    #[prost(message, optional, tag = "2")]
    pub resource: ::core::option::Option<RecognitionResource>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecognitionParameters {
    #[prost(string, tag = "1")]
    pub language: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecognitionResource {
    #[prost(oneof = "recognition_resource::Resource", tags = "1, 2")]
    pub resource: ::core::option::Option<recognition_resource::Resource>,
}
/// Nested message and enum types in `RecognitionResource`.
pub mod recognition_resource {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Model {
        Generic = 0,
        Banking = 1,
        Telco = 2,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Resource {
        #[prost(string, tag = "1")]
        InlineGrammar(::prost::alloc::string::String),
        #[prost(enumeration = "Model", tag = "2")]
        Model(i32),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecognitionResponse {
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod speech_recognizer_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct SpeechRecognizerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SpeechRecognizerClient<tonic::transport::Channel> {
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
    impl<T> SpeechRecognizerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SpeechRecognizerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            SpeechRecognizerClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn recognize_stream(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::RecognitionRequest>,
        ) -> Result<tonic::Response<super::RecognitionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/csr_grpc_gateway.SpeechRecognizer/RecognizeStream",
            );
            self.inner
                .client_streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod speech_recognizer_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with SpeechRecognizerServer."]
    #[async_trait]
    pub trait SpeechRecognizer: Send + Sync + 'static {
        async fn recognize_stream(
            &self,
            request: tonic::Request<tonic::Streaming<super::RecognitionRequest>>,
        ) -> Result<tonic::Response<super::RecognitionResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct SpeechRecognizerServer<T: SpeechRecognizer> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: SpeechRecognizer> SpeechRecognizerServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SpeechRecognizerServer<T>
    where
        T: SpeechRecognizer,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/csr_grpc_gateway.SpeechRecognizer/RecognizeStream" => {
                    #[allow(non_camel_case_types)]
                    struct RecognizeStreamSvc<T: SpeechRecognizer>(pub Arc<T>);
                    impl<T: SpeechRecognizer>
                        tonic::server::ClientStreamingService<super::RecognitionRequest>
                        for RecognizeStreamSvc<T>
                    {
                        type Response = super::RecognitionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::RecognitionRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).recognize_stream(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RecognizeStreamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: SpeechRecognizer> Clone for SpeechRecognizerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: SpeechRecognizer> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SpeechRecognizer> tonic::transport::NamedService for SpeechRecognizerServer<T> {
        const NAME: &'static str = "csr_grpc_gateway.SpeechRecognizer";
    }
}
