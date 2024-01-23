#![feature(type_alias_impl_trait)]
#![no_std]
#![no_main]

use common::{flash_from_channel, read_and_queue, MorseCode};
use embassy_executor::{main, task, Spawner};
use embassy_rp::{
    bind_interrupts,
    config::Config,
    gpio::{Level, Output},
    init,
    peripherals::USB,
    usb::{Driver, InterruptHandler},
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_usb::{
    class::cdc_acm::{CdcAcmClass, State},
    driver::EndpointError,
    Builder, UsbDevice,
};
use embedded_io_async::{Error, ErrorKind, ErrorType, Read};
use panic_halt as _;
use static_cell::make_static;

static QUEUE: Channel<ThreadModeRawMutex, MorseCode, 100> = Channel::new();

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = init(Config::default());
    spawner.must_spawn(queue(Usb::new(Driver::new(peripherals.USB, Irqs), spawner)));
    flash_from_channel(&QUEUE, Output::new(peripherals.PIN_25, Level::Low)).await
}

#[task]
async fn queue(usb: Usb) -> ! {
    read_and_queue(&QUEUE, usb).await
}

struct Usb {
    serial: CdcAcmClass<'static, Driver<'static, USB>>,
}

impl Usb {
    const CONFIG: embassy_usb::Config<'static> = {
        let mut config = embassy_usb::Config::new(0x2E8A, 0x000A);
        config.manufacturer = Some("Ryan Meredith");
        config.product = Some("Morse Code Flasher");
        config.serial_number = Some("1");
        config.max_packet_size_0 = 64;
        config.device_class = 0xEF;
        config.device_sub_class = 0x02;
        config.device_protocol = 0x01;
        config.composite_with_iads = true;
        config
    };

    fn new(driver: Driver<'static, USB>, spawner: Spawner) -> Self {
        let mut builder = Builder::new(
            driver,
            Self::CONFIG,
            make_static!([0; 256]),
            make_static!([0; 256]),
            make_static!([0; 256]),
            make_static!([0; 256]),
            make_static!([0; 64]),
        );
        let serial = CdcAcmClass::new(&mut builder, make_static!(State::new()), 64);
        spawner.must_spawn(run_usb(builder.build()));
        Self { serial }
    }
}

impl ErrorType for Usb {
    type Error = MyError;
}

impl Read for Usb {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.serial.read_packet(buf).await.map_err(MyError)
    }
}

#[derive(Debug)]
struct MyError(EndpointError);

impl Error for MyError {
    fn kind(&self) -> ErrorKind {
        match self.0 {
            EndpointError::BufferOverflow => ErrorKind::OutOfMemory,
            EndpointError::Disabled => ErrorKind::NotConnected,
        }
    }
}

#[task]
async fn run_usb(mut usb: UsbDevice<'static, Driver<'static, USB>>) -> ! {
    usb.run().await
}
