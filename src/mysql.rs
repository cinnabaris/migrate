use _mysql::{from_row_opt, Conn, Opts, Transaction};
use _mysql::prelude::GenericConnection;
use chrono::prelude::{DateTime, NaiveDateTime, Utc};

use super::migration;
use super::result::Result;

pub struct Migration {
    params: Opts,
}

impl Migration {
    pub fn new<T: Into<Opts>>(params: T) -> Result<Self> {
        Ok(Migration {
            params: params.into(),
        })
    }

    fn check<T: GenericConnection>(&self, t: &mut T) -> Result<()> {
        try!(t.prep_exec(
            "CREATE TABLE IF NOT EXISTS schema_migrations(version VARCHAR(255) PRIMARY KEY, created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW())",
            ()
        ));
        return Ok(());
    }

    fn open<F>(&self, f: F) -> Result<()>
    where
        F: Fn(&mut Transaction) -> Result<()>,
    {
        let mut db = try!(Conn::new(self.params.clone()));
        let mut t = try!(db.start_transaction(false, None, None));
        try!(self.check(&mut t));
        try!(f(&mut t));
        try!(t.commit());
        Ok(())
    }
}

impl migration::Migration for Migration {
    fn name(&self) -> &'static str {
        "mysql"
    }

    fn up(&self, name: &String, script: &String) -> Result<()> {
        self.open(|t| {
            try!(t.prep_exec(script, ()));
            try!(t.prep_exec(
                "INSERT INTO schema_migrations(version) VALUES(:version)",
                params!{"version" => name}
            ));
            Ok(())
        })
    }
    fn down(&self, name: &String, script: &String) -> Result<()> {
        self.open(|t| {
            try!(t.prep_exec(script, ()));
            try!(t.prep_exec(
                "DELETE FROM schema_migrations WHERE version = :version",
                params!{"version" => name}
            ));
            Ok(())
        })
    }
    fn versions(&self) -> Result<Vec<(String, DateTime<Utc>)>> {
        let mut db = try!(Conn::new(self.params.clone()));
        try!(self.check(&mut db));
        let rows = try!(db.prep_exec(
            "SELECT version, created_at FROM schema_migrations ORDER BY version ASC",
            ()
        ));
        let mut items = Vec::new();
        for row in rows {
            let (version, created_at) = try!(from_row_opt::<(String, NaiveDateTime)>(try!(row)));
            items.push((version, DateTime::<Utc>::from_utc(created_at, Utc)));
        }

        Ok(items)
    }
}
