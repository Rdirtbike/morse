#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::pin::Pin;

use common::{flash_from_channel, read_and_queue, MorseCode};
use embassy_executor::{task, Spawner};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use esp32c3_hal::{
    clock::ClockControl, embassy::init, entry, macros::main, peripherals::Peripherals,
    system::SystemExt, timer::TimerGroup, UsbSerialJtag, IO,
};
use panic_write::PanicHandler;

static QUEUE: Channel<CriticalSectionRawMutex, MorseCode, 100> = Channel::new();

#[main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = Peripherals::take();
    let usb = PanicHandler::new(UsbSerialJtag::new(peripherals.USB_DEVICE));
    let clocks = ClockControl::boot_defaults(peripherals.SYSTEM.split().clock_control).freeze();
    init(&clocks, TimerGroup::new(peripherals.TIMG0, &clocks));
    spawner.must_spawn(queue(usb));
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
async fn queue(mut usb: Pin<PanicHandler<UsbSerialJtag<'static>>>) -> ! {
    read_and_queue(&QUEUE, &mut *usb).await
}
