#![feature(type_alias_impl_trait)]
#![no_std]
#![no_main]

use common::{flash_from_channel, run_queue};
use embassy_executor::{main, Spawner};
use embassy_futures::join::join;
use embassy_rp::{
    config::Config,
    gpio::{Level, Output},
    init,
};
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, channel::Channel};
use panic_halt as _;

#[main]
async fn main(_spawner: Spawner) {
    let peripherals = init(Config::default());
    let queue: Channel<NoopRawMutex, _, 100> = Channel::new();
    join(
        run_queue(&queue),
        flash_from_channel(&queue, Output::new(peripherals.PIN_25, Level::Low)),
    )
    .await;
}
