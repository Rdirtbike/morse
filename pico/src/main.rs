#![feature(type_alias_impl_trait)]
#![no_std]
#![no_main]

use common::{flash_from_channel, run_queue, MorseCode};
use embassy_executor::{main, task, Spawner};
use embassy_rp::{
    config::Config,
    gpio::{Level, Output},
    init,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use panic_halt as _;

static QUEUE: Channel<CriticalSectionRawMutex, MorseCode, 100> = Channel::new();

#[main]
async fn main(spawner: Spawner) {
    let peripherals = init(Config::default());
    spawner.spawn(sender()).unwrap_or_else(|_| unreachable!());
    flash_from_channel(&QUEUE, Output::new(peripherals.PIN_25, Level::Low)).await;
}

#[task]
async fn sender() {
    run_queue(&mut &QUEUE).await
}
