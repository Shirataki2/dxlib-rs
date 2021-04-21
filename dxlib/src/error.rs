use thiserror::Error;

pub type Result<T> = std::result::Result<T, DxLibError>;

#[derive(Debug, Error)]
pub enum DxLibError {
    #[error("Failed to Initialize")]
    InitializeFailed,
    #[error("Non Zero Returned")]
    NonZeroReturned,
    #[error("Null Returned")]
    NullReturned,
    #[error("Failed to process message")]
    MessageProcessingFailed,
    #[error("Failed to convert path to C-string")]
    InvalidPath,
    #[error("{0} must be between {1} and {2}")]
    InvalidRange(String, i32, i32),
    #[error("String conversion failed: {0}")]
    IntoStringError(#[from] std::ffi::IntoStringError),
    #[error("Interior null character found")]
    NulError(#[from] std::ffi::NulError),
}

pub trait I32CodeExt: Sized {
    fn ensure_zero(&self) -> Result<()>;
    fn ensure_positive(&self) -> Result<Self>;
    fn ensure_not_minus1(&self) -> Result<Self>;
}

impl I32CodeExt for i32 {
    fn ensure_zero(&self) -> Result<()> {
        if *self == 0 {
            Ok(())
        } else {
            Err(DxLibError::NonZeroReturned)
        }
    }

    fn ensure_positive(&self) -> Result<i32> {
        if *self >= 0 {
            Ok(*self)
        } else {
            Err(DxLibError::NonZeroReturned)
        }
    }

    fn ensure_not_minus1(&self) -> Result<i32> {
        if *self != -1 {
            Ok(*self)
        } else {
            Err(DxLibError::NonZeroReturned)
        }
    }
}
