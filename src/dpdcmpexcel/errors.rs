use std::any::Any;

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

    #[error("Join Thread error")]
    Boxed(Box<dyn Any + Send + 'static>)
}

pub type DpdResult<T> = Result<T, DpdError>;
