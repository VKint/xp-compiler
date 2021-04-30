use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompileError {
    #[error("unsupported language: {0}")]
    UnsupportedLang(String),
    #[error("unsupported function: {0}")]
    UnsupportedCall(String),
    #[error("invalid args")]
    InvalidArgs,
}
