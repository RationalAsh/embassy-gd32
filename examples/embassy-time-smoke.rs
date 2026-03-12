#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_gd32::gd32f425::interrupt;
use embassy_gd32::time_driver;
use embassy_time::Timer;
use panic_probe as _;
use rtt_target::{rprintln, rtt_init_print};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    rtt_init_print!();
    time_driver::init();

    let mut count: u32 = 0;

    loop {
        count = count.wrapping_add(1);
        rprintln!("embassy-time-smoke tick={}", count);
        Timer::after_millis(500).await;
    }
}

#[interrupt]
fn TIMER2() {
    time_driver::on_interrupt();
}
