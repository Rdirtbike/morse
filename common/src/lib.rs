#![no_std]

use core::{convert::Infallible, future::Future};

use embassy_sync::{blocking_mutex::raw::RawMutex, channel::Channel};
use embassy_time::{Duration, Instant, Timer};
use embedded_hal::digital::OutputPin;

const TIME_UNIT: Duration = Duration::from_millis(200);

pub enum MorseCode {
    EndOfChar = 0b00,
    Dot = 0b01,
    Space = 0b10,
    Dash = 0b11,
}

pub trait Queue {
    fn queue_code(&mut self, code: MorseCode) -> impl Future<Output = ()>;

    fn queue_char(&mut self, c: u8) -> impl Future<Output = ()> {
        async move {
            match c {
                b'a' | b'A' => {
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b'b' | b'B' => {
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b'c' | b'C' => {
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b'd' | b'D' => {
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b'e' | b'E' => {
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b'f' | b'F' => {
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b'g' | b'G' => {
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b'h' | b'H' => {
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b'i' | b'I' => {
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b'j' | b'J' => {
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b'k' | b'K' => {
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b'l' | b'L' => {
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b'm' | b'M' => {
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b'n' | b'N' => {
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b'o' | b'O' => {
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b'p' | b'P' => {
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b'q' | b'Q' => {
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b'r' | b'R' => {
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b's' | b'S' => {
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b't' | b'T' => {
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b'u' | b'U' => {
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b'v' | b'V' => {
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b'w' | b'W' => {
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b'x' | b'X' => {
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b'y' | b'Y' => {
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b'z' | b'Z' => {
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dash).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::Dot).await;
                    self.queue_code(MorseCode::EndOfChar).await;
                }
                b' ' => {
                    self.queue_code(MorseCode::Space).await;
                }
                _ => {}
            }
        }
    }

    fn queue_string(&mut self, s: &[u8]) -> impl Future<Output = ()> {
        async move {
            for c in s {
                self.queue_char(*c).await;
            }
        }
    }
}

impl<M: RawMutex, const N: usize> Queue for Channel<M, MorseCode, N> {
    fn queue_code(&mut self, code: MorseCode) -> impl Future<Output = ()> {
        self.send(code)
    }
}

impl<M: RawMutex, const N: usize> Queue for &Channel<M, MorseCode, N> {
    fn queue_code(&mut self, code: MorseCode) -> impl Future<Output = ()> {
        self.send(code)
    }
}

pub async fn run_queue(mut queue: impl Queue) {
    loop {
        queue.queue_string(b"Hello world").await;
    }
}

pub async fn flash_from_channel<M: RawMutex, const N: usize>(
    channel: &Channel<M, MorseCode, N>,
    mut pin: impl OutputPin<Error = Infallible>,
) {
    let mut now = Instant::now();
    loop {
        // Get next code, reset time if we have to wait.
        let code = if let Ok(code) = channel.try_receive() {
            code
        } else {
            let code = channel.receive().await;
            now = Instant::now();
            code
        };

        // Set output pin high if needed.
        if let MorseCode::Dot | MorseCode::Dash = code {
            pin.set_high().unwrap_or_default();
        }

        // Wait for the right amount of time.
        now += TIME_UNIT;
        if let MorseCode::Dash | MorseCode::Space = code {
            now += 2 * TIME_UNIT
        }
        Timer::at(now).await;

        // Set output pin low.
        pin.set_low().unwrap_or_default();

        // Wait for the time between flashes.
        now += TIME_UNIT;
        Timer::at(now).await;
    }
}
