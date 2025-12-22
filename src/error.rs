use alloy_json_rpc::RpcError;
use alloy_transport::TransportErrorKind;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MultiProviderError {
    #[error("no providers configured")]
    NoProviders,

    #[error("quorum not reached: needed {required} matching responses, got {received} out of {total} providers")]
    QuorumNotReached {
        required: usize,
        received: usize,
        total: usize,
    },

    #[error("all providers failed: {0}")]
    AllProvidersFailed(String),

    #[error("transport error: {0}")]
    Transport(#[from] alloy_transport::TransportError),

    #[error("invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("failed to initialize provider for {url}: {reason}")]
    InitializationFailed { url: String, reason: String },

    #[error("health check failed: {0}")]
    HealthCheckFailed(String),

    #[error("request timed out after {0:?}")]
    Timeout(std::time::Duration),

    #[error("invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),
}

impl MultiProviderError {
    pub fn into_rpc_error(self) -> RpcError<TransportErrorKind> {
        TransportErrorKind::custom(self)
    }
}

pub type Result<T> = std::result::Result<T, MultiProviderError>;
