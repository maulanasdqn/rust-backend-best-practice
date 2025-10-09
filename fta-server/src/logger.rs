use anyhow::Result;
use std::env;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

#[derive(Debug, Clone, Copy)]
pub enum LogFormat {
    Pretty,
    Json,
    Compact,
}

impl LogFormat {
    fn from_env() -> Self {
        match env::var("LOG_FORMAT")
            .unwrap_or_else(|_| "pretty".to_string())
            .to_lowercase()
            .as_str()
        {
            "json" => Self::Json,
            "compact" => Self::Compact,
            _ => Self::Pretty,
        }
    }
}

#[allow(clippy::too_many_lines)]
pub fn init_logger() -> Result<()> {
    let log_format = LogFormat::from_env();

    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| {
        "info,sqlx=warn,tower_http=debug,axum=debug,fta_server=debug".to_string()
    });

    let env_filter = EnvFilter::try_new(&log_level)?;

    let log_to_file = env::var("LOG_TO_FILE")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);

    match log_format {
        LogFormat::Pretty => {
            let fmt_layer = fmt::layer()
                .with_target(true)
                .with_thread_ids(false)
                .with_thread_names(false)
                .with_file(true)
                .with_line_number(true)
                .with_level(true)
                .with_ansi(true)
                .with_span_events(FmtSpan::CLOSE)
                .pretty();

            if log_to_file {
                let file_appender = tracing_appender::rolling::daily("logs", "app.log");
                let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

                tracing_subscriber::registry()
                    .with(env_filter)
                    .with(fmt_layer)
                    .with(
                        fmt::layer()
                            .with_writer(non_blocking)
                            .with_ansi(false)
                            .json(),
                    )
                    .init();
            } else {
                tracing_subscriber::registry()
                    .with(env_filter)
                    .with(fmt_layer)
                    .init();
            }
        },
        LogFormat::Json => {
            let fmt_layer = fmt::layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true)
                .with_level(true)
                .with_span_events(FmtSpan::CLOSE)
                .json();

            if log_to_file {
                let file_appender = tracing_appender::rolling::daily("logs", "app.log");
                let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

                tracing_subscriber::registry()
                    .with(env_filter)
                    .with(fmt_layer.with_writer(non_blocking))
                    .init();
            } else {
                tracing_subscriber::registry()
                    .with(env_filter)
                    .with(fmt_layer)
                    .init();
            }
        },
        LogFormat::Compact => {
            let fmt_layer = fmt::layer()
                .with_target(true)
                .with_thread_ids(false)
                .with_file(false)
                .with_line_number(false)
                .with_level(true)
                .with_ansi(true)
                .with_span_events(FmtSpan::CLOSE)
                .compact();

            if log_to_file {
                let file_appender = tracing_appender::rolling::daily("logs", "app.log");
                let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

                tracing_subscriber::registry()
                    .with(env_filter)
                    .with(fmt_layer)
                    .with(
                        fmt::layer()
                            .with_writer(non_blocking)
                            .with_ansi(false)
                            .json(),
                    )
                    .init();
            } else {
                tracing_subscriber::registry()
                    .with(env_filter)
                    .with(fmt_layer)
                    .init();
            }
        },
    }

    tracing::info!(
        log_format = ?log_format,
        log_level = %log_level,
        log_to_file = log_to_file,
        "Logger initialized"
    );

    Ok(())
}
