extern crate peach_menu;
#[macro_use]
extern crate log;
extern crate env_logger;

use ::std::process;

fn main() {
    // initialize the logger
    env_logger::init();

    // handle errors returned from `run`
    if let Err(e) = peach_menu::run() {
        error!("Application error: {:?}", e);
        process::exit(1);
    }
}
