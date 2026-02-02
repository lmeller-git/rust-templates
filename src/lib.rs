#![no_std]

#[cfg(any(test, bench))]
extern crate std;

pub mod sync;
#[cfg(test)]
mod tests;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(not(any(test, bench)))]
#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {
        crate::sync::hint::spin_loop();
    }
}
