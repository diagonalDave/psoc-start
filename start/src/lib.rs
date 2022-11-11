#![no_std]
#![allow(unused_imports)]
//! This version of start implements a semaphore for signalling
//! between the cores to blink the LEDs on a CY8PROTO-063-BLE
//! board. The blinking has two modes:
//! - set semaphore, both LEDS are on.
//! - unset semaphore, both LEDS blink.
//! Pressing the user button (SW2) changes mode.
//! To use this file with Psoc-creator:
//! 1. Create an empty project for the CYBLE-416045-02 in psoc-creator.
//! 2. Remove the template generated for loop from the
//!    main_cm0p.c and main_cm4.c files.
//! 3. Add include start.h,  start_cm0(); to main_cm0p.c and start_cm4();  to main_cm4.c
//! 4. Run: nu build_debug.nu or nu build_release.nu to generate the
//!         required libs.
//! 5. Clean Build release (or debug) in the Psoc-creator project.
//! 6. In Psoc-creator program the psoc.
extern crate panic_semihosting;
//blinky bits
use cortex_m::{interrupt, interrupt::free, peripheral::Peripherals};
use psoc6_hal::{
    delay::Delay,
    drivers::ipc::{ChannelConfig, IntrStructMaskBits, InterruptMaskBits, semaphore::{Semaphore, UnInit}},
    prelude::*,
    psoc::Psoc,
    gpio::{FilterSelect, EdgeSelect, PinInterrupt},
};

#[no_mangle]
#[cfg(not(armv7em))]
pub extern "C" fn start_cm0() -> ! {
    
    //
    let mut psoc = Psoc::new();
   
    //initialise the pins.
    let (mut red_led, user_sw) = free(|cs| {
        (
            psoc.gpio.p6_3.into_strong_output(cs),
            psoc.gpio.p0_4.into_pull_up_input(cs),
        )
    });
    //Add some method of timing off and on. Will be modified
    //once the clock/timer module is working.
    let cp = Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST);

    // Config values for the semaphore.
    // These values configure which interrupts are used when
    // Channel events occur.
    let config = ChannelConfig {
        struct_release: IntrStructMaskBits::intr_struct15,
        struct_notify: IntrStructMaskBits::intr_struct15,
        intr_release_mask: InterruptMaskBits::cpuss_interrupt15,
        intr_notify_mask: InterruptMaskBits::cpuss_interrupt14,
    };
    
    let mut sem = Semaphore::<UnInit>::configure(&mut psoc.ipc_intr.syscall,config).unwrap();
    sem.start(&mut psoc.ipc.semaphores).unwrap();
    //set the semaphore for the first non system channel.
    sem.set(64).unwrap();
    
    //add interrupts and configure:
    //FilterSelect::Disable avoids configuration of the 50ns glitch filter.
    // EdgeSelect::Falling causes the interrupt to be triggered on the falling edge.
    // i.e. when the pin gets pulled low.
    user_sw.configure_interrupt(FilterSelect::Disable, EdgeSelect::Falling);
    //Safety: Ok because it is not being called within a CriticalSection. 
    unsafe{interrupt::enable()};


     //start the cm4 core.    
    psoc.cpuss.cm4_reset(); // this is important even when starting up
    psoc.cpuss.cm4_enable(0x10080000);//base address for the start of the CM4 flash.
    
    loop {
        //Check the semaphore
        if sem.flag_is_set(64){
            if user_sw.interrupt() == PinInterrupt::Set {
                sem.clear(64).unwrap();
                user_sw.clear_interrupt();
            }else{
                //the semaphore is set with no switch interrupt.
                red_led.set_low().unwrap();
            }
        }else {
            // When the semaphore isn't set.
            if user_sw.interrupt() == PinInterrupt::Set{
                //The button is pressed.
                sem.set(64).unwrap();
                user_sw.clear_interrupt();
            }else{
                // If it the switch interrupt hasn't been set
                // blink away.
                red_led.set_low().unwrap();
                delay.delay_ms(1000u32);
                red_led.set_high().unwrap();
                delay.delay_ms(1000u32);
            }
        }
    }
}

#[no_mangle]
#[cfg(armv7em)]
pub extern "C" fn start_cm4() -> ! {
    // Get a new psoc to configure and provide peripherals for the CM4 core.
    let mut psoc = Psoc::new();
    // Configure the pin wiht the green led.
    let mut green_led = free(|cs| {psoc.gpio.p7_1.into_strong_output(cs)});

    //Add some method of timing off and on. Will be modified
    //once the clock/timer module is working.
    let cp = Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST);

    //Configure a CM4 semaphore.
    let sem = Semaphore::<UnInit>::configure(&mut psoc.ipc.semaphores).unwrap();
    //Safety: Ok because it is not being called within a CriticalSection. 
    unsafe{interrupt::enable()};
    
    loop {
        //Check the semaphore.
         if sem.flag_is_set(64){
            green_led.set_low().unwrap();
        }else{
            green_led.set_low().unwrap();
            delay.delay_ms(1000u32);
            green_led.set_high().unwrap();
            delay.delay_ms(1000u32);
        }
    }
}
