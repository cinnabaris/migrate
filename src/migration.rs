use chrono::prelude::{DateTime, Utc};

use super::result::Result;

pub trait Migration {
    fn name(&self) -> &'static str;
    fn up(&self, name: &String, script: &String) -> Result<()>;
    fn down(&self, name: &String, script: &String) -> Result<()>;
    fn versions(&self) -> Result<Vec<(String, DateTime<Utc>)>>;
}
