#![feature(type_alias_impl_trait)]
#![no_std]
#![no_main]

use common::{flash_from_channel, read_and_queue, MorseCode};
use core::{fmt::Write, mem, panic::PanicInfo, slice};
use cortex_m::interrupt;
use embassy_executor::{main, task, Spawner};
use embassy_rp::{
    bind_interrupts,
    clocks::{ClockConfig, SysClkSrc},
    config::Config,
    gpio::{Level, Output},
    init,
    pac::{
        xosc::vals::{CtrlFreqRange, Enable},
        XIP_CTRL, XOSC,
    },
    peripherals::USB,
    rom_data::reset_to_usb_boot,
    usb::{Driver, InterruptHandler},
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_usb::{
    class::cdc_acm::{CdcAcmClass, State},
    driver::EndpointError,
    Builder, UsbDevice,
};
use embedded_io_async::{Error, ErrorKind, ErrorType, Read};
use static_cell::make_static;

static QUEUE: Channel<ThreadModeRawMutex, MorseCode, 100> = Channel::new();

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = init(Config::new({
        let mut clocks = ClockConfig::crystal(12_000_000);
        clocks.rosc = None;
        clocks.peri_clk_src = None;
        clocks.adc_clk = None;
        clocks.rtc_clk = None;
        clocks.ref_clk.div = 4;
        clocks.sys_clk.src = SysClkSrc::PllUsb;
        if let Some(xosc) = &mut clocks.xosc {
            xosc.sys_pll = None;
        }
        clocks
    }));
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

struct Cursor<'a>(&'a mut [u8]);

impl<'a> Write for Cursor<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let len = bytes.len();
        if len < self.0.len() {
            let (a, b) = mem::take(&mut self.0).split_at_mut(len);
            a.copy_from_slice(bytes);
            self.0 = b;
            Ok(())
        } else {
            Err(core::fmt::Error)
        }
    }
}

#[panic_handler]
fn handle_panic(panic_info: &PanicInfo) -> ! {
    interrupt::disable();

    XIP_CTRL.ctrl().write(|reg| {
        reg.set_power_down(false);
        reg.set_en(false);
    });

    _ = write!(
        Cursor(unsafe { slice::from_raw_parts_mut(0x1500_0000 as *mut u8, 0x4000) }),
        "{}\n\0",
        panic_info
    );

    if !XOSC.status().read().stable() {
        XOSC.startup()
            .write(|reg| reg.set_delay((12_000 + 128) / 256));
        XOSC.ctrl().write(|reg| {
            reg.set_freq_range(CtrlFreqRange::_1_15MHZ);
            reg.set_enable(Enable::ENABLE);
        });
        while !XOSC.status().read().stable() {}
    }

    loop {
        reset_to_usb_boot(1 << 25, 0);
    }
}
