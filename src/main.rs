#![no_std]
#![cfg_attr(not(test), no_main)]
#[cfg(test)]
extern crate std;

mod sync;
#[cfg(test)]
mod tests;

#[cfg(not(test))]
#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    loop {
        crate::sync::hint::spin_loop();
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {
        crate::sync::hint::spin_loop();
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
