use std::fs;
use std::os::unix::fs::OpenOptionsExt;
use std::io::Read;
use std::path::{Path, PathBuf};

use postgres;
use url::Url;
use chrono::prelude::{DateTime, Utc};

use super::migration::Migration;
use super::result::{Error, Result};
use super::{mysql, postgresql, sqlite};

pub struct Scheme {
    mig: Box<Migration>,
}

impl Scheme {
    pub fn new(m: Box<Migration>) -> Self {
        Scheme { mig: m }
    }

    pub fn migrate(&self) -> Result<()> {
        let mut files = Vec::new();
        for it in try!(fs::read_dir(self.migrations_dir())) {
            files.push(try!(it));
        }
        files.sort_by_key(|it| it.path());

        let versions = try!(self.mig.versions());
        let items: Vec<String> = versions
            .iter()
            .map(|it| {
                let &(ref vr, ref _ts) = it;
                vr.to_string()
            })
            .collect();

        for it in files {
            if let Some(path) = it.path().file_name() {
                if let Some(name) = path.to_str() {
                    let name = name.to_string();
                    info!("find migration {}", name);
                    if !items.contains(&name) {
                        let mut file = it.path().join("up");
                        file.set_extension(self.migrations_ext());
                        let mut fd = try!(fs::OpenOptions::new().read(true).open(file));
                        let mut buf = String::new();
                        try!(fd.read_to_string(&mut buf));
                        debug!("run migration: {}\n{}", name, buf);
                        try!(self.mig.up(&name, &buf));
                    }
                }
            }
        }

        Ok(())
    }

    pub fn rollback(&self) -> Result<Option<String>> {
        match try!(self.mig.versions()).pop() {
            Some(it) => {
                let (vr, _ts) = it;
                let name = vr.to_string();
                let mut file = self.migrations_dir().join(&name).join("down");
                file.set_extension(self.migrations_ext());
                let mut fd = try!(fs::OpenOptions::new().read(true).open(file));
                let mut buf = String::new();
                try!(fd.read_to_string(&mut buf));
                debug!("run rollback: {}\n{}", name, buf);
                try!(self.mig.down(&name, &buf));
                Ok(Some(name))
            }
            None => Ok(None),
        }
    }

    pub fn versions(&self) -> Result<Vec<(String, DateTime<Utc>)>> {
        self.mig.versions()
    }

    pub fn create(&self, name: &String) -> Result<()> {
        let now: DateTime<Utc> = Utc::now();
        let root = self.migrations_dir()
            .join(format!("{}_{}", now.format("%Y%m%d%H%M%S"), name));
        try!(fs::create_dir_all(&root));
        let files = vec!["up", "down"];
        for n in files.into_iter() {
            let mut file = root.join(n);
            file.set_extension(self.migrations_ext());
            info!("generate file {}", file.display());
            try!(
                fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .mode(0o600)
                    .open(file)
            );
        }

        return Ok(());
    }

    // ---------
    fn migrations_dir(&self) -> PathBuf {
        Path::new("db").join(self.mig.name()).join("migrations")
    }

    fn migrations_ext(&self) -> &'static str {
        "sql"
    }
}

pub fn parse(url: String) -> Result<Scheme> {
    let uri = try!(Url::parse(&url));
    match uri.scheme() {
        "postgresql" => {
            let db = try!(postgresql::Migration::new(url, postgres::TlsMode::None));
            Ok(Scheme::new(Box::new(db)))
        }
        "mysql" => {
            let db = try!(mysql::Migration::new(url));
            Ok(Scheme::new(Box::new(db)))
        }
        "sqlite" => {
            let db = try!(sqlite::Migration::new(Path::new(uri.path()).to_path_buf()));
            Ok(Scheme::new(Box::new(db)))
        }
        _d => Err(Error::BadDriver(_d.to_string())),
    }
}
