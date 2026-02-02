#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod sync;
#[cfg(test)]
mod tests;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
