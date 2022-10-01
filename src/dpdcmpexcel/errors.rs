#[derive(Debug, thiserror::Error)]
pub enum DpdError {
    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("format string error")]
    Fmt(#[from] std::fmt::Error),

    #[error("Procesing excel error")]
    Excel(#[from] calamine::Error),

    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Processing error: {0}")]
    Processing(String),
}

pub type DpdResult<T> = Result<T, DpdError>;
