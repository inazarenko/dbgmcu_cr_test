#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

fn dbgmcu_cr() ->  u32 {
    let ptr = 0xE004_2004 as * const u32;
    unsafe { core::ptr::read_volatile(ptr) }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(embassy_stm32::Config::default());

    let mut led = Output::new(p.PE11, Level::Low, Speed::Low);

    loop {
        let dbg_sleep = (dbgmcu_cr() & 1) != 0;
        led.set_level(if dbg_sleep { Level::High } else { Level::Low });
        Timer::after(Duration::from_millis(500)).await;
    }
}
