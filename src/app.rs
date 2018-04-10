use clap;

use super::result::Result;
use super::scheme;

pub fn run() -> Result<()> {
    let create = clap::SubCommand::with_name("create")
        .about("Generate database migration file(up.sql,down.sql)")
        .arg(
            clap::Arg::with_name("name")
                .short("n")
                .help("Migration's name")
                .required(true)
                .takes_value(true),
        );

    let migrate = clap::SubCommand::with_name("migrate")
        .about("Migrate the DB to the most recent version available");
    let rollback = clap::SubCommand::with_name("rollback").about("Roll back the version by 1");
    let version = clap::SubCommand::with_name("version")
        .about("Dump the migration status for the current DB");

    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .after_help(env!("CARGO_PKG_HOMEPAGE"))
        .arg(
            clap::Arg::with_name("url")
                .short("u")
                .help(
                    "Database connection url: \
            \n postgresql://user[:password]@host[:port][/database][?param1=val1[[&param2=val2]...]]
            \n mysql://root:password@localhost:3307
            \n sqlite.db
            ",
                )
                .required(true)
                .takes_value(true),
        )
        .subcommand(create)
        .subcommand(migrate)
        .subcommand(rollback)
        .subcommand(version)
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("create") {
        if let Some(name) = matches.value_of("name") {
            if let Some(url) = matches.value_of("url") {
                let sch = try!(scheme::parse(url.to_string()));
                return sch.create(&name.to_string());
            }
        }
    }

    if let Some(_) = matches.subcommand_matches("migrate") {
        if let Some(url) = matches.value_of("url") {
            let sch = try!(scheme::parse(url.to_string()));
            return sch.migrate();
        }
    }
    if let Some(_) = matches.subcommand_matches("rollback") {
        if let Some(url) = matches.value_of("url") {
            let sch = try!(scheme::parse(url.to_string()));
            try!(sch.rollback());
            return Ok(());
        }
    }
    if let Some(_) = matches.subcommand_matches("versions") {
        if let Some(url) = matches.value_of("url") {
            let sch = try!(scheme::parse(url.to_string()));
            println!("{:<32}\t{}", "VERSION", "CREATED AT");
            for (v, t) in try!(sch.versions()) {
                println!("{:<32}\t{}", v, t.to_rfc2822());
            }
            return Ok(());
        }
    }

    Ok(())
}
