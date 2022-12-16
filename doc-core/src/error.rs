use std::convert::Infallible;
use std::fmt::{Debug, Display, Formatter};
use std::io::ErrorKind::Other;

pub type DocResult<T> = Result<T, DocError>;

pub enum DocError {
    StdErr(std::fmt::Error),
    IoErr(std::io::Error),
    SoftError(ErrorType),
}

#[derive(Debug)]
pub enum ErrorType {
    NotMatch,
    Format(String),
    Index(String),
}

impl Display for DocError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DocError::StdErr(err) => write!(f, "{}", err),
            DocError::IoErr(err) => write!(f, " {}", err),
            DocError::SoftError(err) => write!(f, "{:?}", err),
        }
    }
}

impl Debug for DocError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DocError::StdErr(err) => write!(f, "{}", err),
            DocError::IoErr(err) => write!(f, "{}", err),
            DocError::SoftError(err) => write!(f, "{:?}", err),
        }
    }
}

impl From<std::io::Error> for DocError {
    fn from(value: std::io::Error) -> Self {
        Self::IoErr(value)
    }
}

impl From<Infallible> for DocError {
    fn from(value: Infallible) -> Self {
        Self::IoErr(std::io::Error::new(Other, value))
    }
}

impl From<std::fmt::Error> for DocError {
    fn from(value: std::fmt::Error) -> Self {
        Self::StdErr(value)
    }
}

impl From<ErrorType> for DocError {
    fn from(value: ErrorType) -> Self {
        Self::SoftError(value)
    }
}

impl std::error::Error for DocError {}
