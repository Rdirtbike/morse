#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use common::{flash_from_channel, MorseCode, Queue};
use embassy_executor::{task, Spawner};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel, mutex::Mutex};
use embedded_hal_async::digital::Wait;
use embedded_io_async::Read;
use esp32c3_hal::{
    clock::ClockControl,
    embassy::init,
    entry,
    gpio::{Gpio9, Input, PullUp},
    macros::main,
    peripherals::Peripherals,
    system::SystemExt,
    timer::TimerGroup,
    UsbSerialJtag, IO,
};
use esp_backtrace as _;

static QUEUE: Channel<CriticalSectionRawMutex, MorseCode, 100> = Channel::new();
static SENDING: Mutex<CriticalSectionRawMutex, ()> = Mutex::new(());

#[main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = Peripherals::take();
    let clocks = ClockControl::boot_defaults(peripherals.SYSTEM.split().clock_control).freeze();
    init(&clocks, TimerGroup::new(peripherals.TIMG0, &clocks));
    let pins = IO::new(peripherals.GPIO, peripherals.IO_MUX).pins;
    spawner.must_spawn(queue(UsbSerialJtag::new(peripherals.USB_DEVICE)));
    spawner.must_spawn(button(pins.gpio9.into_pull_up_input()));
    flash_from_channel(&QUEUE, pins.gpio7.into_push_pull_output()).await
}

#[task]
async fn queue(mut usb: UsbSerialJtag<'static>) -> ! {
    let mut buf = [0u8; 64];
    loop {
        if let Ok(n) = usb.read(&mut buf).await {
            let _lock = SENDING.lock().await;
            (&QUEUE).queue_string(&buf[..n]).await;
        }
    }
}

#[task]
async fn button(mut pin: Gpio9<Input<PullUp>>) -> ! {
    loop {
        pin.wait_for_falling_edge().await.unwrap_or_default();
        let _lock = SENDING.lock().await;
        (&QUEUE).queue_string(b"Hello world").await;
    }
}
