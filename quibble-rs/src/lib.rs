#![no_std]
#![feature(vec_into_raw_parts, c_variadic)]

use core::ffi::{c_char, CStr};

use uefi::table::{Boot, SystemTable};

extern crate alloc;

pub mod allocator;
pub mod ini;

extern "C" {
    static mut systable: SystemTable<Boot>;
}

#[no_mangle]
pub extern "C" fn add_10(i: u64) -> u64 {
    i + 10
}
