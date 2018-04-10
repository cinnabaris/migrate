use std::{error, fmt, io, result, string};

use clap;
use postgres;
use rusqlite;
use _mysql;
use url;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Postgre(postgres::Error),
    Io(io::Error),
    StringFromUtf8(string::FromUtf8Error),
    Clap(clap::Error),
    Rusqlite(rusqlite::Error),
    Mysql(_mysql::Error),
    MysqlFromRow(_mysql::FromRowError),
    UrlParse(url::ParseError),
    BadDriver(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Postgre(ref err) => err.fmt(f),
            Error::Io(ref err) => err.fmt(f),
            Error::StringFromUtf8(ref err) => err.fmt(f),
            Error::Clap(ref err) => err.fmt(f),
            Error::Rusqlite(ref err) => err.fmt(f),
            Error::Mysql(ref err) => err.fmt(f),
            Error::MysqlFromRow(ref err) => err.fmt(f),
            Error::UrlParse(ref err) => err.fmt(f),
            Error::BadDriver(ref desc) => write!(f, "{}", desc),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Postgre(ref err) => err.description(),
            Error::Io(ref err) => err.description(),
            Error::StringFromUtf8(ref err) => err.description(),
            Error::Clap(ref err) => err.description(),
            Error::Rusqlite(ref err) => err.description(),
            Error::Mysql(ref err) => err.description(),
            Error::MysqlFromRow(ref err) => err.description(),
            Error::UrlParse(ref err) => err.description(),
            Error::BadDriver(ref desc) => &desc[..],
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Postgre(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
            Error::StringFromUtf8(ref err) => Some(err),
            Error::Clap(ref err) => Some(err),
            Error::Rusqlite(ref err) => Some(err),
            Error::Mysql(ref err) => Some(err),
            Error::MysqlFromRow(ref err) => Some(err),
            Error::UrlParse(ref err) => Some(err),
            Error::BadDriver(ref _desc) => None,
        }
    }
}

impl From<postgres::Error> for Error {
    fn from(err: postgres::Error) -> Error {
        Error::Postgre(err)
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

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Error {
        Error::Rusqlite(err)
    }
}

impl From<_mysql::Error> for Error {
    fn from(err: _mysql::Error) -> Error {
        Error::Mysql(err)
    }
}

impl From<_mysql::FromRowError> for Error {
    fn from(err: _mysql::FromRowError) -> Error {
        Error::MysqlFromRow(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::UrlParse(err)
    }
}
