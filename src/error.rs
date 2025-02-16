use std::net::AddrParseError;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Usage: jawbreaker <url>")]
    CliUsage,

    #[error("HTTP request failed: {0}")]
    Reqwest(String),

    #[error("Invalid address: {0}")]
    AddressError(String),

    #[error("DNS error: {0}")]
    Dns(String),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err.to_string())
    }
}

impl From<AddrParseError> for Error {
    fn from(err: AddrParseError) -> Self {
        Error::AddressError(err.to_string())
    }
}
