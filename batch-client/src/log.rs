#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct IsoTime;

impl tracing_subscriber::fmt::time::FormatTime for IsoTime {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let now = chrono::Local::now().naive_utc();
        let formatted = now.format("%Y-%m-%d %H:%M:%S%.3f");
        write!(w, "[{}] ", formatted)
    }
}

pub fn init_logger(log_level: &str) {
    let env_filter = match std::env::var("RUST_LOG") {
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
    };
    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_timer(IsoTime)
        .with_env_filter(env_filter)
        .with_ansi(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Couldn't set tracing global subscriber");
}
