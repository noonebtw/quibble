use core::{
    ffi::{c_char, c_void},
    ptr::null_mut,
};

use uefi::table::boot::MemoryType;

use crate::systable;

#[no_mangle]
pub extern "C" fn efi_malloc(size: usize) -> *mut c_char {
    unsafe {
        systable
            .boot_services()
            .allocate_pool(MemoryType::LOADER_DATA, size)
    }
    .expect("failed to allocate memory. OOM.")
    .cast()
}

#[no_mangle]
pub extern "C" fn efi_free(ptr: *mut c_void) {
    if !ptr.is_null() {
        unsafe {
            _ = systable.boot_services().free_pool(ptr.cast());
        }
    }
}
