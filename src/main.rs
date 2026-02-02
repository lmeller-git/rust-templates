#![no_std]
#![cfg_attr(not(any(test, bench)), no_main)]

#[cfg(not(any(test, bench)))]
#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    loop {
        bin_template::sync::hint::spin_loop();
    }
}

#[cfg(bench)]
fn main() {}
