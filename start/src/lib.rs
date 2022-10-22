#![no_std]

extern crate panic_semihosting;
use cortex_m::peripheral::Peripherals;
use psoc6_hal::{
    delay::Delay,
    psoc::Psoc,
    prelude::*,
};

#[no_mangle]
#[cfg(armv6m)]
pub extern "C" fn start_cm0()->!{
    let psoc = Psoc::new();
    let mut red_led = psoc.gpio.p6_3.into_strong_output();
    let cp = Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST);
    loop{
        red_led.set_low().unwrap();
        delay.delay_ms(1000u32);
        red_led.set_high().unwrap();
        delay.delay_ms(1000u32);
    }
    
}
#[no_mangle]
#[cfg(armv7em)]
pub extern "C" fn start_cm4()-> !{
    let psoc = Psoc::new();
    let mut green_led = psoc.gpio.p7_1.into_strong_output();
    let cp = Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST);
    loop{
        green_led.set_low().unwrap();
        delay.delay_ms(1000u32);
        green_led.set_high().unwrap();
        delay.delay_ms(1000u32);
    }
}
