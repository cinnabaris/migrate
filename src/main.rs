extern crate env_logger;
extern crate migrate;
#[macro_use]
extern crate log;

fn main() {
    env_logger::init();
    if let Err(e) = migrate::run() {
        error!("{:?}", e);
    }
}
