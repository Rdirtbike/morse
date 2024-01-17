#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use common::{flash_from_channel, read_and_queue};
use embassy_executor::{main, Spawner};
use embassy_futures::join::join;
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, channel::Channel};
use esp32c3_hal::{
    clock::ClockControl, embassy::init, peripherals::Peripherals, system::SystemExt,
    timer::TimerGroup, UsbSerialJtag, IO,
};
use panic_halt as _;

#[main(entry = "esp32c3_hal::entry")]
async fn main(_spawner: Spawner) {
    let peripherals = Peripherals::take();
    let clocks = ClockControl::boot_defaults(peripherals.SYSTEM.split().clock_control).freeze();
    let queue: Channel<NoopRawMutex, _, 100> = Channel::new();
    init(&clocks, TimerGroup::new(peripherals.TIMG0, &clocks).timer0);
    join(
        read_and_queue(&queue, UsbSerialJtag::new(peripherals.USB_DEVICE)),
        flash_from_channel(
            &queue,
            IO::new(peripherals.GPIO, peripherals.IO_MUX)
                .pins
                .gpio7
                .into_push_pull_output(),
        ),
    )
    .await;
}
