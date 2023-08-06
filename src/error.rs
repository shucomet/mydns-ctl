use std::{error, fmt};

#[derive(Clone, Debug)]
pub enum MydnsCtlError {
    Auth(String),
    Curl(String),
}

impl From<curl::Error> for MydnsCtlError {
    fn from(e: curl::Error) -> Self {
        Self::Curl(e.to_string())
    }
}

impl fmt::Display for MydnsCtlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use MydnsCtlError::*;
        match self {
            Auth(s) => write!(f, "Error(Auth): {}", s),
            Curl(s) => write!(f, "Error(Curl): {}", s),
        }
    }
}

impl error::Error for MydnsCtlError {}
