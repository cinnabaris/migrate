use std::{error, fmt, io, result, string};

use clap;
use postgres;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    PostgreSql(postgres::Error),
    Io(io::Error),
    StringFromUtf8(string::FromUtf8Error),
    Clap(clap::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::PostgreSql(ref err) => err.fmt(f),
            Error::Io(ref err) => err.fmt(f),
            Error::StringFromUtf8(ref err) => err.fmt(f),
            Error::Clap(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::PostgreSql(ref err) => err.description(),
            Error::Io(ref err) => err.description(),
            Error::StringFromUtf8(ref err) => err.description(),
            Error::Clap(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::PostgreSql(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
            Error::StringFromUtf8(ref err) => Some(err),
            Error::Clap(ref err) => Some(err),
        }
    }
}

impl From<postgres::Error> for Error {
    fn from(err: postgres::Error) -> Error {
        Error::PostgreSql(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Error {
        Error::StringFromUtf8(err)
    }
}

impl From<clap::Error> for Error {
    fn from(err: clap::Error) -> Error {
        Error::Clap(err)
    }
}
