use crate::speechcenter_tts_v1::speech_synthesizer_client::SpeechSynthesizerClient;
use crate::speechcenter_tts_v1::synthesis_voice::{SynthesisUnion, Voice};
use crate::speechcenter_tts_v1::{
    AudioFormat as SynthesisFormat, SynthesisRequest, SynthesisVoice, VoiceSamplingRate,
};
use crate::{Result, SpeechCenterError};
use bytes::Bytes;
use std::error::Error;
use std::str::FromStr;
use tonic::codegen::InterceptedService;
use tonic::metadata::{Ascii, MetadataValue};
use tonic::service::Interceptor;
use tonic::transport::{Channel, ClientTlsConfig, Uri};
use tonic::{Request, Status};

#[derive(Clone, Debug, PartialEq)]
pub enum Speaker {
    EnUsTommy,
    EnUsAnnie,
    EsEsAurora,
    EsEsDavid,
    PtBrLuma,
    CaCaDavid,
}

impl Speaker {
    pub fn from_name(name: &str, language: &str) -> Result<Self> {
        match (
            name.to_lowercase().as_str(),
            language.to_lowercase().as_str(),
        ) {
            ("tommy", "en-us") => Ok(Self::EnUsTommy),
            ("annie", "en-us") => Ok(Self::EnUsAnnie),
            ("aurora", "es-es") => Ok(Self::EsEsAurora),
            ("david", "es-es") => Ok(Self::EsEsDavid),
            ("luma", "pt-br") => Ok(Self::PtBrLuma),
            ("david", "ca-ca") => Ok(Self::CaCaDavid),
            _ => Err(SpeechCenterError::Unknown(format!(
                "Nonexistent Speaker for Name/LanguageTag combination: {}/{}",
                name, language
            ))),
        }
    }

    pub fn to_voice(self) -> Voice {
        match self {
            Self::EnUsTommy => Voice::EnUsTommy,
            Self::EnUsAnnie => Voice::EnUsAnnie,
            Self::EsEsAurora => Voice::EsEsAurora,
            Self::EsEsDavid => Voice::EsEsDavid,
            Self::PtBrLuma => Voice::PtBrLuma,
            Self::CaCaDavid => Voice::CaCaDavid,
        }
    }
}

#[derive(Clone, Debug)]
pub enum SampleRate {
    Khz8,
}

impl TryFrom<u32> for SampleRate {
    type Error = SpeechCenterError;

    fn try_from(sample_rate: u32) -> Result<Self, Self::Error> {
        match sample_rate {
            8000 => Ok(Self::Khz8),
            _ => Err(SpeechCenterError::Unknown(format!(
                "Nonexistent Sample Rate option for {}",
                sample_rate
            ))),
        }
    }
}

impl From<SampleRate> for u32 {
    fn from(sample_rate: SampleRate) -> u32 {
        match sample_rate {
            SampleRate::Khz8 => 8000,
        }
    }
}

impl SampleRate {
    pub fn to_sampling_rate(self) -> VoiceSamplingRate {
        match self {
            Self::Khz8 => VoiceSamplingRate::VoiceSamplingRate8khz,
        }
    }
}

#[derive(Clone, Debug)]
pub enum AudioFormat {
    WavLpcmS16le,
    RawLpcmS16le,
}

impl AudioFormat {
    pub fn from_str(audio_encoding: &str, audio_header: &str) -> Result<Self> {
        match (
            audio_encoding.to_lowercase().as_str(),
            audio_header.to_lowercase().as_str(),
        ) {
            ("pcm", "wav") => Ok(Self::WavLpcmS16le),
            ("pcm", "raw") => Ok(Self::RawLpcmS16le),
            _ => Err(SpeechCenterError::Unknown(format!(
                "Nonexistent Audio Format for Encoding/Header combination: {}/{}",
                audio_encoding, audio_header
            ))),
        }
    }

    pub fn to_synthesis_format(self) -> SynthesisFormat {
        match self {
            Self::WavLpcmS16le => SynthesisFormat::WavLpcmS16le,
            Self::RawLpcmS16le => SynthesisFormat::RawLpcmS16le,
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

#[derive(Debug)]
pub struct Client {
    inner: SpeechSynthesizerClient<InterceptedService<Channel, AddAuthorizationInterceptor>>,
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
            SpeechCenterError::Connection(format!(
                "Could not connect [url={}] {}",
                url,
                e.source().map(|e| e.to_string()).unwrap_or_default()
            ))
        })?;

        let interceptor = AddAuthorizationInterceptor::new(credentials)?;
        let c = SpeechSynthesizerClient::with_interceptor(channel, interceptor);
        Ok(Self { inner: c })
    }

    fn synthesis_request(
        speaker: Speaker,
        sample_rate: SampleRate,
        audio_format: AudioFormat,
        text: String,
    ) -> SynthesisRequest {
        let voice = speaker.to_voice();
        let voice_sampling_rate = sample_rate.to_sampling_rate();
        let synthesis_format = audio_format.to_synthesis_format();
        SynthesisRequest {
            voice: Some(SynthesisVoice {
                synthesis_union: Some(SynthesisUnion::Voice(i32::from(voice))),
            }),
            voice_sampling_rate: i32::from(voice_sampling_rate),
            audio_format: i32::from(synthesis_format),
            text,
        }
    }

    pub async fn synthesize(
        &mut self,
        speaker: Speaker,
        sample_rate: SampleRate,
        audio_format: AudioFormat,
        text: &str,
    ) -> Result<Bytes> {
        let r = Self::synthesis_request(speaker, sample_rate, audio_format, text.to_string());
        let r =
            self.inner.synthesize(Request::new(r)).await.map_err(|e| {
                SpeechCenterError::Synthesis(format!("Error in synthesis: {:?}", e))
            })?;
        let res = r.get_ref();
        Ok(Bytes::from(res.audio.to_owned()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_empty_url() {
        let error = Client::new("", "")
            .await
            .expect_err("Should not be able to create a client");
        assert!(matches!(error, SpeechCenterError::Unknown(_)));
    }

    #[tokio::test]
    async fn test_connection_error() {
        let error = Client::new("http://127.0.0.1:9999", "")
            .await
            .expect_err("Should not be able to connect anywhere");
        assert!(matches!(error, SpeechCenterError::Connection(_)));
    }
}
