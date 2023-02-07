//! This shows how to write text to uart0.
//! You can see the output with `espflash` if you provide the `--monitor` option

#![feature(stmt_expr_attributes)]
#![no_std]
#![no_main]

use core::fmt::Write;

use esp32c6_hal::{
    clock::ClockControl,
    peripherals::Peripherals,
    prelude::*,
    timer::TimerGroup,
    Rtc,
    Uart,
};
use esp_backtrace as _;
use esp_riscv_rt::entry;
use nb::block;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.PCR.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let mut uart0 = Uart::new(peripherals.UART0);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut timer0 = timer_group0.timer0;
    let _wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let _wdt1 = timer_group1.wdt;

    let _rtc = Rtc::new(peripherals.LP_CLKRST);

    timer0.start(1u64.secs());

    loop {
        writeln!(uart0, "Hello world!").unwrap();
        block!(timer0.wait()).unwrap();
    }
}
