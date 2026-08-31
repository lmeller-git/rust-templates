#![no_std]
#![cfg_attr(not(any(test, bench)), no_main)]

#[cfg(not(any(test, bench)))]
#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    loop {
        {{crate_name | snake_case}}::sync::hint::spin_loop();
    }
}

#[cfg(bench)]
fn main() {}
