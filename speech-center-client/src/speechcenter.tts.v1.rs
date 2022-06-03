#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynthesisRequest {
    /// Voice to use for the synthesis request.
    #[prost(message, optional, tag = "1")]
    pub voice: ::core::option::Option<SynthesisVoice>,
    /// Text to synthesize to audio.
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
    /// Voice sampling rate (VOICE_SAMPLING_RATE_8KHZ by default).
    #[prost(enumeration = "VoiceSamplingRate", tag = "3")]
    pub voice_sampling_rate: i32,
    /// Audio format for the synthesized audio (AUDIO_FORMAT_WAV_LPCM_S16LE by default).
    #[prost(enumeration = "AudioFormat", tag = "4")]
    pub audio_format: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynthesisVoice {
    #[prost(oneof = "synthesis_voice::SynthesisUnion", tags = "1, 2")]
    pub synthesis_union: ::core::option::Option<synthesis_voice::SynthesisUnion>,
}
/// Nested message and enum types in `SynthesisVoice`.
pub mod synthesis_voice {
    /// Voices supported by language and name.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Voice {
        /// American English male voice.
        EnUsTommy = 0,
        /// American English female voice.
        EnUsAnnie = 1,
        /// Spanish female voice.
        EsEsAurora = 2,
        /// Spanish male voice.
        EsEsDavid = 3,
        /// Brazilian female voice.
        PtBrLuma = 4,
        /// Catalan male voice.
        CaCaDavid = 5,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SynthesisUnion {
        /// Client custom voice. Currently not implemented.
        #[prost(string, tag = "1")]
        CustomVoice(::prost::alloc::string::String),
        /// One of the voices from Voice.
        #[prost(enumeration = "Voice", tag = "2")]
        Voice(i32),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynthesisResponse {
    /// Returned audio data in the requested AudioFormat.
    #[prost(bytes = "vec", tag = "1")]
    pub audio: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AudioFormat {
    /// Linear Pulse-Code Modulation with signed 16 bit samples, little endian byte order, with a WAV header.
    WavLpcmS16le = 0,
    /// Linear Pulse-Code Modulation with signed 16 bit samples, little endian byte order, without any header.
    RawLpcmS16le = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VoiceSamplingRate {
    /// Voice sampling rate is 8 kHz.
    VoiceSamplingRate8khz = 0,
}
#[doc = r" Generated client implementations."]
pub mod speech_synthesizer_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct SpeechSynthesizerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SpeechSynthesizerClient<tonic::transport::Channel> {
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
    impl<T> SpeechSynthesizerClient<T>
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
        ) -> SpeechSynthesizerClient<InterceptedService<T, F>>
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
            SpeechSynthesizerClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn synthesize(
            &mut self,
            request: impl tonic::IntoRequest<super::SynthesisRequest>,
        ) -> Result<tonic::Response<super::SynthesisResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/speechcenter.tts.v1.SpeechSynthesizer/Synthesize",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod speech_synthesizer_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with SpeechSynthesizerServer."]
    #[async_trait]
    pub trait SpeechSynthesizer: Send + Sync + 'static {
        async fn synthesize(
            &self,
            request: tonic::Request<super::SynthesisRequest>,
        ) -> Result<tonic::Response<super::SynthesisResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct SpeechSynthesizerServer<T: SpeechSynthesizer> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: SpeechSynthesizer> SpeechSynthesizerServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SpeechSynthesizerServer<T>
    where
        T: SpeechSynthesizer,
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
                "/speechcenter.tts.v1.SpeechSynthesizer/Synthesize" => {
                    #[allow(non_camel_case_types)]
                    struct SynthesizeSvc<T: SpeechSynthesizer>(pub Arc<T>);
                    impl<T: SpeechSynthesizer> tonic::server::UnaryService<super::SynthesisRequest>
                        for SynthesizeSvc<T>
                    {
                        type Response = super::SynthesisResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SynthesisRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).synthesize(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SynthesizeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
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
    impl<T: SpeechSynthesizer> Clone for SpeechSynthesizerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: SpeechSynthesizer> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SpeechSynthesizer> tonic::transport::NamedService for SpeechSynthesizerServer<T> {
        const NAME: &'static str = "speechcenter.tts.v1.SpeechSynthesizer";
    }
}
