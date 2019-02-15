use std::process;
extern crate inventory_mgt;

use structopt::StructOpt;

fn main() {
    let config = inventory_mgt::InventoryConfig::from_args();

    if let Err(e) = inventory_mgt::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
