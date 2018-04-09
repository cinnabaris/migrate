use postgres::{Connection, GenericConnection, TlsMode};
use postgres::params::IntoConnectParams;
use chrono::prelude::{DateTime, Utc};

use super::migration;
use super::result::Result;

pub struct Migration {
    db: Connection,
}

impl Migration {
    pub fn new<T>(params: T, tls: TlsMode) -> Result<Self>
    where
        T: IntoConnectParams,
    {
        let db = try!(Connection::connect(params, tls));
        Ok(Migration { db: db })
    }

    fn check<T: GenericConnection>(&self, t: &T) -> Result<()> {
        try!(t.execute(
            "CREATE TABLE IF NOT EXISTS schema_migrations(version VARCHAR(255) PRIMARY KEY, created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW())",
            &[]
        ));
        return Ok(());
    }
}

impl migration::Migration for Migration {
    fn name(&self) -> &'static str {
        "postgresql"
    }

    fn up(&self, name: &String, script: &String) -> Result<()> {
        let t = try!(self.db.transaction());
        try!(self.check(&t));
        try!(t.batch_execute(script));
        try!(t.execute("INSERT INTO schema_migrations(version) VALUES($1)", &[name]));
        try!(t.commit());
        Ok(())
    }
    fn down(&self, name: &String, script: &String) -> Result<()> {
        let t = try!(self.db.transaction());
        try!(self.check(&t));
        try!(t.batch_execute(script));
        try!(t.execute("DELETE FROM schema_migrations WHERE version = $1", &[name]));
        try!(t.commit());
        Ok(())
    }
    fn versions(&self) -> Result<Vec<(String, DateTime<Utc>)>> {
        try!(self.check(&self.db));
        let mut items = Vec::new();
        for row in &self.db.query(
            "SELECT version, created_at FROM schema_migrations ORDER BY version ASC",
            &[],
        )? {
            let version: String = row.get("version");
            let created_at: DateTime<Utc> = row.get("created_at");
            items.push((version, created_at))
        }

        Ok(items)
    }
}
