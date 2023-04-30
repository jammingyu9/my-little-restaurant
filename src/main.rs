use my_little_waiter::restaurant;
use log::{info, error};
use std::process::exit;
use std::{thread, time};

fn main() {
    let mut termFlag: bool = false;

    if !restaurant::prepare() {
        error!("Failed to prepare the restaurant!");
        exit(1);
    }
    info!("Restaurant is ready to open!");

    if !restaurant::open(&mut termFlag) {
        error!("Failed to open the restaurant!");
    }

    while !termFlag {
        thread::sleep(time::Duration::from_millis(1));
    }

    info!("Closing the restaurant!");
    restaurant::close();
}
