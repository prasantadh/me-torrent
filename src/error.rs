#![allow(dead_code)]

use std::string::FromUtf8Error;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Bencode(serde_bencode::Error),
    FromUtf8(FromUtf8Error),
    InfoNotFound,
    MissingTracker,
    Reqwest(reqwest::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO(value)
    }
}

impl From<serde_bencode::Error> for Error {
    fn from(value: serde_bencode::Error) -> Self {
        Error::Bencode(value)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Error::FromUtf8(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::Reqwest(value)
    }
}
