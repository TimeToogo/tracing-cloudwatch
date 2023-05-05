#[cfg(feature = "rusoto")]
mod rusoto;

use async_trait::async_trait;

use crate::{dispatch::LogEvent, export::LogDestination};

#[async_trait]
pub trait CloudWatchClient {
    async fn put_logs(&self, dest: LogDestination, logs: Vec<LogEvent>)
        -> Result<(), PutLogsError>;
}

#[derive(Debug, thiserror::Error)]
pub enum PutLogsError {
    #[error("{message}")]
    LogDestinationNotFound { message: String },
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub struct NoopClient {}

#[async_trait]
impl CloudWatchClient for NoopClient {
    async fn put_logs(&self, _: LogDestination, _: Vec<LogEvent>) -> Result<(), PutLogsError> {
        Ok(())
    }
}

impl NoopClient {
    pub fn new() -> Self {
        Self {}
    }
}
