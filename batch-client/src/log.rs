use std::str::FromStr;
use tracing_subscriber::fmt::format::{DefaultFields, Format, Full};
use tracing_subscriber::fmt::Subscriber;
use tracing_subscriber::EnvFilter;

type LoggerSubscriber =
    Subscriber<DefaultFields, Format<Full, IsoTime>, EnvFilter, fn() -> std::io::Stdout>;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct IsoTime;

impl tracing_subscriber::fmt::time::FormatTime for IsoTime {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let now = chrono::Local::now().naive_utc();
        let formatted = now.format("%Y-%m-%d %H:%M:%S%.3f");
        write!(w, "[{}] ", formatted)
    }
}

pub struct Logger {
    _local_subscriber: tracing::dispatcher::DefaultGuard,
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

impl Logger {
    pub fn new() -> Self {
        let local_subscriber =
            tracing::subscriber::set_default(Self::subscriber(tracing::Level::INFO));
        tracing_log::env_logger::init();
        Self {
            _local_subscriber: local_subscriber,
        }
    }

    pub fn set_global_subscriber(&mut self, log_level: &str) {
        let level = tracing::Level::from_str(log_level).expect("Invalid log level value");
        self._local_subscriber = tracing::subscriber::set_default(Self::subscriber(level));
        tracing::subscriber::set_global_default(Self::subscriber(level))
            .expect("Couldn't set tracing global subscriber");
    }

    fn subscriber(log_level: tracing::Level) -> LoggerSubscriber {
        tracing_subscriber::fmt::Subscriber::builder()
            .with_timer(IsoTime)
            .with_env_filter(Self::build_env_filter(log_level))
            .with_ansi(true)
            .finish()
    }

    fn build_env_filter(log_level: tracing::Level) -> String {
        match std::env::var("RUST_LOG") {
            Ok(v) => v,
            Err(_) => {
                let modules = vec!["batch_client", "speech_center_client"];
                let log_level = log_level.to_string();
                let env_filter = modules
                    .iter()
                    .map(|x| format!("{}={}", x, log_level))
                    .collect::<Vec<String>>()
                    .join(",");
                std::env::set_var("RUST_LOG", &env_filter);
                env_filter
            }
        }
    }
}
