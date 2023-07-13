#![no_std]
#![feature(abi_efiapi)]
#![no_main]

use core::{ffi::c_void, panic::PanicInfo};

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

extern "win64" {
    fn efi_main(image: *mut (), st: *mut ()) -> usize;
}

#[no_mangle]
fn __main(image: *mut (), st: *mut ()) -> usize {
    unsafe { efi_main(image, st) }
}
