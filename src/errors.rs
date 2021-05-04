use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenerationError {
    #[error("argument parse error")]
    ParseError,
    #[error("unsupported function")]
    UnsupportedCall,
}

#[derive(Error, Debug)]
pub enum CompileError {
    #[error("unsupported language: {0}")]
    UnsupportedLang(String),
    #[error("unsupported function: {0}")]
    UnsupportedCall(String),
    #[error("invalid args")]
    InvalidArgs,
}

impl CompileError {
    pub(crate) fn from_generation(e: GenerationError, s: String) -> Self {
        match e {
            GenerationError::ParseError => Self::InvalidArgs,
            GenerationError::UnsupportedCall => Self::UnsupportedCall(s),
        }
    }
}
