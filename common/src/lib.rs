#![no_std]

use core::{convert::Infallible, future::Future};

use embassy_time::{Duration, Instant, Timer};
use embedded_hal::digital::OutputPin;

const TIME_UNIT: Duration = Duration::from_millis(200);

pub enum MorseCode {
    EndOfChar = 0b00,
    Dot = 0b01,
    Space = 0b10,
    Dash = 0b11,
}

pub trait Flasher {
    fn flash_code(&mut self, code: MorseCode) -> impl Future<Output = ()>;

    fn flash_char(&mut self, c: u8) -> impl Future<Output = ()> {
        async move {
            match c {
                b'a' | b'A' => {
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b'b' | b'B' => {
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b'c' | b'C' => {
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b'd' | b'D' => {
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b'e' | b'E' => {
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b'f' | b'F' => {
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b'g' | b'G' => {
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b'h' | b'H' => {
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b'i' | b'I' => {
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b'j' | b'J' => {
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b'k' | b'K' => {
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b'l' | b'L' => {
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b'm' | b'M' => {
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b'n' | b'N' => {
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b'o' | b'O' => {
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b'p' | b'P' => {
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b'q' | b'Q' => {
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b'r' | b'R' => {
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b's' | b'S' => {
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b't' | b'T' => {
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b'u' | b'U' => {
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b'v' | b'V' => {
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b'w' | b'W' => {
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b'x' | b'X' => {
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b'y' | b'Y' => {
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b'z' | b'Z' => {
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dash).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::Dot).await;
                    self.flash_code(MorseCode::EndOfChar).await;
                }
                b' ' => {
                    self.flash_code(MorseCode::Space).await;
                }
                _ => {}
            }
        }
    }

    fn flash_string(&mut self, s: &[u8]) -> impl Future<Output = ()> {
        async move {
            for c in s {
                self.flash_char(*c).await;
            }
        }
    }
}

pub struct EmbassyFlasher<Pin> {
    pub pin: Pin,
    now: Instant,
}

impl<Pin> EmbassyFlasher<Pin> {
    pub fn new(pin: Pin) -> Self {
        Self {
            pin,
            now: Instant::now(),
        }
    }

    pub fn reset_time(&mut self) {
        self.now = Instant::now();
    }
}

impl<Pin: OutputPin<Error = Infallible>> Flasher for EmbassyFlasher<Pin> {
    fn flash_code(&mut self, code: MorseCode) -> impl Future<Output = ()> {
        async move {
            if let MorseCode::Dot | MorseCode::Dash = code {
                self.pin.set_high().unwrap_or_default();
            }
            self.now += TIME_UNIT;
            if let MorseCode::Dash | MorseCode::Space = code {
                self.now += 2 * TIME_UNIT;
            }
            Timer::at(self.now).await;
            self.pin.set_low().unwrap_or_default();
            self.now += TIME_UNIT;
            Timer::at(self.now).await;
        }
    }
}

pub async fn run(mut flasher: EmbassyFlasher<impl OutputPin<Error = Infallible>>) {
    loop {
        flasher.flash_string(b"Hello world").await;
        flasher.now += 10 * TIME_UNIT;
        Timer::at(flasher.now).await;
    }
}
