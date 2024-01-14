#![feature(type_alias_impl_trait)]
#![no_std]
#![no_main]

use common::{EmbassyFlasher, Flasher};
use embassy_executor::{main, Spawner};
use embassy_rp::{
    config::Config,
    gpio::{Level, Output},
    init,
};
use panic_halt as _;

#[main]
async fn main(_spawner: Spawner) {
    let peripherals = init(Config::default());
    let mut flasher = EmbassyFlasher::new(Output::new(peripherals.PIN_25, Level::Low));
    loop {
        flasher.flash_string(b"Hello world").await;
    }
}
