use reqwest;
use semver;

pub enum Error {
    ReqwestError,
    RegexError,
    SemverError,
}

impl From<reqwest::Error> for Error {
    fn from(_: reqwest::Error) -> Error {
        Error::ReqwestError
    }
}

impl From<semver::Error> for Error {
    fn from(_: semver::Error) -> Error {
        Error::SemverError
    }
}