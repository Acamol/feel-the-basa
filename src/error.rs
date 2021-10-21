use semver;
use tinyget;

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