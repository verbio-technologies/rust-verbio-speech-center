use crate::csr_grpc_gateway::recognition_request::RequestUnion;
use crate::csr_grpc_gateway::recognition_resource::{Model, Resource};
use crate::csr_grpc_gateway::speech_recognizer_client::SpeechRecognizerClient;
use crate::csr_grpc_gateway::{
    RecognitionInit, RecognitionParameters, RecognitionRequest, RecognitionResource,
};
use crate::{Result, SpeechCenterError};
use std::str::FromStr;
use tonic::codegen::InterceptedService;
use tonic::metadata::{Ascii, MetadataValue};
use tonic::service::Interceptor;
use tonic::transport::{Channel, ClientTlsConfig, Uri};
use tonic::{Request, Status};

#[derive(Clone, Debug)]
pub enum Topic {
    Generic,
    Banking,
    Telco,
}

impl Topic {
    pub fn from_name(name: &str) -> Result<Self> {
        match name.to_lowercase().as_str() {
            "generic" => Ok(Topic::Generic),
            "banking" => Ok(Topic::Banking),
            "telco" => Ok(Topic::Telco),
            _ => Err(SpeechCenterError::Unknown(format!(
                "Unknown model name: {}",
                name
            ))),
        }
    }

    pub fn to_model(self) -> Model {
        match self {
            Topic::Generic => Model::Generic,
            Topic::Banking => Model::Banking,
            Topic::Telco => Model::Telco,
        }
    }
}

pub struct AddAuthorizationInterceptor {
    credentials: MetadataValue<Ascii>,
}

impl AddAuthorizationInterceptor {
    pub fn new(token: &str) -> Result<Self> {
        let credentials = tonic::metadata::MetadataValue::from_str(token).map_err(|e| {
            SpeechCenterError::Unknown(format!(
                "Error converting credentials to MetadataValue: {}",
                e
            ))
        })?;
        Ok(Self { credentials })
    }
}

impl Interceptor for AddAuthorizationInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        request
            .metadata_mut()
            .insert("authorization", self.credentials.clone());
        Ok(request)
    }
}

pub struct Client {
    inner: SpeechRecognizerClient<InterceptedService<Channel, AddAuthorizationInterceptor>>,
}

impl Client {
    pub async fn new(url: &str, credentials: &str) -> Result<Self> {
        let uri = Uri::from_str(url).map_err(|e| {
            SpeechCenterError::Unknown(format!(
                "Error building Uri from string [uri={}]: {}",
                url, e
            ))
        })?;
        let channel = Channel::builder(uri)
            .tls_config(ClientTlsConfig::default())
            .map_err(|e| SpeechCenterError::Unknown(format!("Error setting up tls: {:?}", e)))?;
        let channel = channel.connect().await.map_err(|e| {
            SpeechCenterError::Connection(format!("Error in connection [url={}]: {:?}", url, e))
        })?;

        let interceptor = AddAuthorizationInterceptor::new(credentials)?;
        let c = SpeechRecognizerClient::with_interceptor(channel, interceptor);
        Ok(Self { inner: c })
    }

    pub async fn recognise_with_topic(
        &mut self,
        language: &str,
        topic: Topic,
        audio: Vec<u8>,
    ) -> Result<String> {
        let model = topic.to_model();
        let initial = RecognitionRequest {
            request_union: Some(RequestUnion::Init(RecognitionInit {
                parameters: Some(RecognitionParameters {
                    language: language.to_string(),
                }),
                resource: Some(RecognitionResource {
                    resource: Some(Resource::Model(i32::from(model))),
                }),
            })),
        };
        self.recognise(audio, initial).await
    }

    pub async fn recognise_with_grammar(
        &mut self,
        grammar: &str,
        language: &str,
        audio: Vec<u8>,
    ) -> Result<String> {
        let initial = RecognitionRequest {
            request_union: Some(RequestUnion::Init(RecognitionInit {
                parameters: Some(RecognitionParameters {
                    language: language.to_string(),
                }),
                resource: Some(RecognitionResource {
                    resource: Some(Resource::InlineGrammar(grammar.to_string())),
                }),
            })),
        };
        self.recognise(audio, initial).await
    }

    async fn recognise(&mut self, audio: Vec<u8>, initial: RecognitionRequest) -> Result<String> {
        let audio_req = RecognitionRequest {
            request_union: Some(RequestUnion::Audio(audio)),
        };
        let s = async_stream::stream! {
            yield initial;
            yield audio_req;
        };

        let r = self
            .inner
            .recognize_stream(Request::new(s))
            .await
            .map_err(|e| {
                SpeechCenterError::Recognision(format!("Error in recognision: {:?}", e))
            })?;
        let res = r.get_ref();
        Ok(res.text.to_string())
    }
}
