#![no_std]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[repr(C)]
pub struct MyType {
    i: u64,
    f: f32,
}

#[no_mangle]
pub extern "C" fn add_10(i: u64) -> u64 {
    i + 10
}

#[no_mangle]
pub extern "C" fn my_type_new() -> MyType {
    MyType { i: 2, f: 3.4 }
}
