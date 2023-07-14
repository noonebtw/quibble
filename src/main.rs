#![no_std]
#![feature(lang_items)]
#![no_main]

extern crate alloc;

pub use quibble_rs::{
    add_10,
    allocator::{efi_free, efi_malloc},
    say_hello,
};

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

use uefi::{
    table::{Boot, SystemTable},
    Handle, Status,
};

extern "win64" {
    fn efi_main(image: Handle, st: SystemTable<Boot>) -> Status;
}

#[no_mangle]
extern "efiapi" fn __main(image: Handle, mut st: SystemTable<Boot>) -> Status {
    unsafe {
        st.boot_services().set_image_handle(image);
    }
    uefi_services::init(&mut st).unwrap();
    log::info!("this is add_10: {}", quibble_rs::add_10(3));
    unsafe { efi_main(image, st) }
}
