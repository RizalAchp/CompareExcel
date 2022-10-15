#[derive(Debug, thiserror::Error)]
pub enum DpdError {
    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("format string error")]
    Fmt(#[from] std::fmt::Error),

    #[error("Procesing excel error")]
    Excel(#[from] calamine::Error),

    #[error("Processing csv error")]
    Csv(#[from] csv::Error),

    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Processing error: {0}")]
    Processing(String),

    #[allow(unused)]
    #[error("Unkown Error")]
    Unknown,
}

pub type DpdResult<T> = Result<T, DpdError>;
