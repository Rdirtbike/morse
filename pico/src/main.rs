#![feature(type_alias_impl_trait)]
#![no_std]
#![no_main]

use common::{flash_from_channel, run_queue, MorseCode};
use embassy_executor::{main, task, Executor, Spawner};
use embassy_rp::{
    config::Config,
    gpio::{Level, Output},
    init,
    multicore::{spawn_core1, Stack},
    peripherals::PIN_25,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use panic_halt as _;
use static_cell::make_static;

static QUEUE: Channel<CriticalSectionRawMutex, MorseCode, 100> = Channel::new();

#[main]
async fn main(_spawner: Spawner) {
    let peripherals = init(Config::default());
    spawn_core1(
        peripherals.CORE1,
        make_static!(Stack::<4096>::new()),
        || {
            make_static!(Executor::new())
                .run(|s| s.must_spawn(test(Output::new(peripherals.PIN_25, Level::Low))))
        },
    );
    run_queue(&QUEUE).await
}

#[task]
async fn test(pin: Output<'static, PIN_25>) {
    flash_from_channel(&QUEUE, pin).await
}
