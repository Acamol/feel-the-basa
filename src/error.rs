use semver;
use minreq;
use regex;

pub enum Error {
    ReqwestError,
    RegexError,
    SemverError,
}

impl From<minreq::Error> for Error {
    fn from(_: minreq::Error) -> Error {
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