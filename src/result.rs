use std::{error, fmt, result};

use postgres;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    PostgreSql(postgres::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::PostgreSql(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::PostgreSql(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::PostgreSql(ref err) => Some(err),
        }
    }
}

impl From<postgres::Error> for Error {
    fn from(err: postgres::Error) -> Error {
        Error::PostgreSql(err)
    }
}
