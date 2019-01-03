use std::fmt;
use std::error;

#[derive(Debug)]
pub enum GiphyError {
    HttpError(reqwest::Error),
    ParsingError(serde_json::Error),
}

impl From<reqwest::Error> for GiphyError {
    fn from(e: reqwest::Error) -> Self {
        GiphyError::HttpError(e)
    }
}

impl From<serde_json::Error> for GiphyError {
    fn from(e: serde_json::Error) -> Self {
        GiphyError::ParsingError(e)
    }
}

impl fmt::Display for GiphyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error while calling giphy API!")
    }
}

impl error::Error for GiphyError {
    fn description(&self) -> &str {
        "Error while calling giphy API!"
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            GiphyError::HttpError(e) => Some(e),
            GiphyError::ParsingError(e) => Some(e)
        }
    }
}