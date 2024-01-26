#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use common::{flash_from_channel, read_and_queue, MorseCode};
use embassy_executor::{task, Spawner};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use esp32c3_hal::{
    clock::ClockControl, embassy::init, entry, macros::main, peripherals::Peripherals,
    system::SystemExt, timer::TimerGroup, UsbSerialJtag, IO,
};
use esp_backtrace as _;

static QUEUE: Channel<CriticalSectionRawMutex, MorseCode, 100> = Channel::new();

#[main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = Peripherals::take();
    let clocks = ClockControl::boot_defaults(peripherals.SYSTEM.split().clock_control).freeze();
    init(&clocks, TimerGroup::new(peripherals.TIMG0, &clocks));
    spawner.must_spawn(queue(UsbSerialJtag::new(peripherals.USB_DEVICE)));
    flash_from_channel(
        &QUEUE,
        IO::new(peripherals.GPIO, peripherals.IO_MUX)
            .pins
            .gpio7
            .into_push_pull_output(),
    )
    .await
}

#[task]
async fn queue(usb: UsbSerialJtag<'static>) -> ! {
    read_and_queue(&QUEUE, usb).await
}
