//! Blinks the LED on a Adafruit QT Py RP2040 board
//!
//! This will blink on-board LED.
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{digital::v2::OutputPin, PwmPin};
use embedded_time::rate::*;
use panic_halt as _;
use qt_py_rp2040::{
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        pac,
        sio::Sio,
        watchdog::Watchdog,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};
use smart_leds::SmartLedsWrite;
use ws2812_timer_delay::Ws2812;

const LOW: u16 = 0;

// The maximum PWM value (i.e. LED brightness) we want
const HIGH: u16 = 25000;
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GD25Q64CS;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    pins.neopixel_power
        .into_push_pull_output()
        .set_high()
        .unwrap();

    let timer_clock = rp2040_hal::timer::Timer::count_down();
    let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.PM);
    timer.start(3.mhz());

    let neopixel_data = pins.neopixel_data.into_push_pull_output().into();
    let mut neopixel = Ws2812::new(timer, neopixel_data);

    loop {
        for j in 0..255u8 {
            neopixel
                .write(
                    [smart_leds::hsv::hsv2rgb(smart_leds::hsv::Hsv {
                        hue: j,
                        sat: 255,
                        val: 16,
                    })]
                    .iter()
                    .cloned(),
                )
                .unwrap();
            delay.delay_ms(500);
        }
    }
}
