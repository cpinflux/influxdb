mod cpu_mem_sampler;
mod sender;
mod stats;
pub mod store;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TelemetryError {
    #[error("cannot serialize to JSON: {0}")]
    CannotSerializeJson(#[from] serde_json::Error),

    #[error("failed to get pid: {0}")]
    CannotGetPid(&'static str),

    #[error("cannot send telemetry: {0}")]
    CannotSendToTelemetryServer(#[from] reqwest::Error),
}

pub type Result<T, E = TelemetryError> = std::result::Result<T, E>;
