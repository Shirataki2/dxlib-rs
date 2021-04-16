use thiserror::Error;

pub type Result<T> = std::result::Result<T, DxLibError>;

#[derive(Debug, Error)]
pub enum DxLibError {
    #[error("{0:?}")]
    LibraryCallFailed(#[from] libloading::Error),
    #[error("Failed to Initialize")]
    InitializeFailed,
    #[error("Non Zero Returned")]
    NonZeroReturned,
    #[error("Failed to process message")]
    MessageProcessingFailed,
}
