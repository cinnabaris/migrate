extern crate mysql as _mysql;
extern crate postgres;
extern crate rusqlite;

pub mod result;
pub mod postgresql;
pub mod mysql;
pub mod sqlite;

pub trait Migration {
    fn name() -> &'static str;
}

pub fn run() -> result::Result<()> {
    Ok(())
}
