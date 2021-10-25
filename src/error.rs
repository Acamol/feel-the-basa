use semver;
use tinyget;
use regex;

pub enum Error {
    ReqwestError,
    RegexError,
    SemverError,
}

impl From<tinyget::Error> for Error {
    fn from(_: tinyget::Error) -> Error {
        Error::ReqwestError
    }
}

impl From<semver::Error> for Error {
    fn from(_: semver::Error) -> Error {
        Error::SemverError
    }
}

impl From<regex::Error> for Error {
    fn from(_: regex::Error) -> Error {
        Error::RegexError
    }
}