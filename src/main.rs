#![no_main]
#![no_std]
extern crate alloc;
use core::ffi::*;
use libnds_sys::arm9_bindings::*;
use libnds_sys::*;

fn console_init() {
    unsafe {
        consoleDemoInit();
    }
}

fn console_clear() {
    unsafe {
        consoleClear();
    }
}

fn wait_for_vblank() {
    unsafe {
        swiWaitForVBlank();
    }
}

fn print_second(second: u32) {
    console_clear();

    println!("Second: {}/{}", second, 10);

    wait_for_vblank();
}

fn wait(seconds: u32) {
    let mut frame = 0;
    let mut second = 0;

    while second < seconds {
        if frame % 60 == 0 {
            second += 1;
        }

        wait_for_vblank();

        frame += 1;
    }
}

#[unsafe(no_mangle)]
extern "C" fn main() -> c_int {
    return init();
}

fn init() -> c_int {
    console_init();

    let mut iterations = 1;

    loop {
        print_second(iterations);

        if iterations >= 10 {
            break;
        }

        wait(2);

        iterations += 1;
    }

    wait(1);

    return 0;
}
