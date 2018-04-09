use std::path::PathBuf;

use rusqlite::{Connection, Transaction};
use chrono::prelude::{DateTime, Utc};

use super::migration;
use super::result::Result;

pub struct Migration {
    file: PathBuf,
}

impl Migration {
    pub fn new(file: PathBuf) -> Result<Self> {
        Ok(Migration { file: file })
    }

    fn open<F>(&self, f: F) -> Result<()>
    where
        F: Fn(&Transaction) -> Result<()>,
    {
        let mut db = try!(Connection::open(&self.file));
        let tx = try!(db.transaction());
        try!(tx.execute(
            "CREATE TABLE IF NOT EXISTS schema_migrations(version VARCHAR(255) PRIMARY KEY, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)",
            &[]
        ));
        try!(f(&tx));
        try!(tx.commit());
        return Ok(());
    }
}

impl migration::Migration for Migration {
    fn name(&self) -> &'static str {
        "sqlite"
    }

    fn up(&self, name: &String, script: &String) -> Result<()> {
        self.open(|tx| {
            try!(tx.execute_batch(script));
            try!(tx.execute("INSERT INTO schema_migrations(version) VALUES(?1)", &[name]));

            Ok(())
        })
    }
    fn down(&self, name: &String, script: &String) -> Result<()> {
        self.open(|tx| {
            try!(tx.execute_batch(script));
            try!(tx.execute("DELETE FROM schema_migrations WHERE version = ?1", &[name]));

            Ok(())
        })
    }
    fn versions(&self) -> Result<Vec<(String, DateTime<Utc>)>> {
        try!(self.open(|_tx| Ok(())));
        let db = try!(Connection::open(&self.file));
        let mut stmt = try!(db.prepare(
            "SELECT version, created_at FROM schema_migrations ORDER BY version ASC"
        ));
        let rows = try!(stmt.query_map(&[], |row| {
            let version: String = row.get("version");
            let created_at: DateTime<Utc> = row.get("created_at");
            (version, created_at)
        }));
        let mut items = Vec::new();
        for it in rows {
            items.push(try!(it));
        }

        Ok(items)
    }
}
