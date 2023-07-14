use alloc::{
    ffi::CString,
    string::{String, ToString},
    vec::Vec,
};
use widestring::U16CString;

pub fn u16_cstring_to_ffi(cstr: U16CString) -> *mut u16 {
    cstr.into_raw()
}

pub unsafe fn u16_cstring_from_ffi(ptr: *const u16) -> U16CString {
    U16CString::from_ptr_str(ptr)
}

pub fn str_to_ffi_u16c(cstr: &str) -> *mut u16 {
    u16_cstring_to_ffi(U16CString::from_str(cstr).expect("null byte in string"))
}

pub fn string_to_ffi(cstr: String) -> *mut u8 {
    cstring_to_ffi(CString::new(cstr).expect("null byte in string"))
}

pub fn cstring_to_ffi(cstr: CString) -> *mut u8 {
    cstr.into_raw().cast()
}

pub unsafe fn cstring_from_ffi(ptr: *const u8) -> CString {
    CString::from_raw(ptr as *mut _)
}

pub mod ffi {
    use core::{ffi::c_char, ptr::null_mut};

    use alloc::{boxed::Box, ffi::CString, vec::Vec};

    use super::{
        cstring_from_ffi, cstring_to_ffi, str_to_ffi_u16c, string_to_ffi, u16_cstring_from_ffi,
    };

    #[repr(C)]
    pub struct OperatingSystem {
        display_name: *const c_char,
        display_namew: *const i16,
        system_path: *const c_char,
        options: *mut c_char,
    }

    impl OperatingSystem {
        fn from_os(os: &super::OperatingSystem) -> Self {
            let display_name = string_to_ffi(os.display_name.clone()).cast();
            let display_namew = str_to_ffi_u16c(&os.display_name).cast();

            let system_path = string_to_ffi(os.system_path.clone()).cast();

            let options = os
                .options
                .clone()
                .map(|options| string_to_ffi(options))
                .unwrap_or(null_mut())
                .cast();

            Self {
                display_name,
                system_path,
                options,
                display_namew,
            }
        }

        #[no_mangle]
        pub extern "C" fn operating_system_destroy(this: Self) {
            unsafe {
                if !this.options.is_null() {
                    cstring_from_ffi(this.options.cast());
                }
                cstring_from_ffi(this.display_name.cast());
                cstring_from_ffi(this.system_path.cast());
                u16_cstring_from_ffi(this.display_namew.cast());
                cstring_from_ffi(this.options.cast());
            }
        }
    }

    #[repr(C)]
    pub struct QuibbleOptions {
        timeout: u64,
        default_os: *const u8,
        operating_systems: *mut OperatingSystem,
        operating_systems_len: usize,
        operating_systems_capacity: usize,
    }

    impl QuibbleOptions {
        fn from_options(options: super::Options) -> Self {
            let default_os = options
                .default_os
                .clone()
                .map(|options| string_to_ffi(options))
                .unwrap_or(null_mut())
                .cast();

            let (operating_systems, operating_systems_len, operating_systems_capacity) = options
                .operating_systems
                .into_iter()
                .map(|os| OperatingSystem::from_os(&os))
                .collect::<Vec<_>>()
                .into_raw_parts();

            Self {
                timeout: options.timeout,
                default_os,
                operating_systems,
                operating_systems_len,
                operating_systems_capacity,
            }
        }

        #[no_mangle]
        pub extern "C" fn quibble_options_destroy(this: *const Self) {
            unsafe {
                let this = Box::from_raw(this.cast_mut());
                if !this.default_os.is_null() {
                    cstring_from_ffi(this.default_os);
                }
                _ = Vec::from_raw_parts(
                    this.operating_systems,
                    this.operating_systems_len,
                    this.operating_systems_capacity,
                );
            }
        }

        #[no_mangle]
        pub extern "C" fn parse_quibble_options(data: *const u8, len: usize) -> *const Self {
            let contents =
                core::str::from_utf8(unsafe { core::slice::from_raw_parts(data.cast(), len) })
                    .expect("ini file contents were not proper utf8.");
            let options =
                super::Options::parse_from_bytes(contents).expect("failed to parse ini file.");

            Box::leak(Box::new(Self::from_options(options)))
        }
    }
}

#[derive(Debug)]
struct OperatingSystem {
    display_name: String,
    system_path: String,
    options: Option<String>,
}

impl OperatingSystem {
    fn parse_from_section(display_name: &str, section: &ini::Properties) -> anyhow::Result<Self> {
        let system_path = section
            .get("SystemPath")
            .ok_or(anyhow::anyhow!(
                "Operating System section did not have system path field but is required to."
            ))?
            .to_string();

        let options = section.get("Options").map(|s| s.to_string());

        Ok(Self {
            system_path,
            options,
            display_name: display_name.to_string(),
        })
    }
}

#[derive(Debug)]
struct Options {
    timeout: u64,
    default_os: Option<String>,
    operating_systems: Vec<OperatingSystem>,
}

impl Options {
    pub fn parse_from_bytes(contents: &str) -> anyhow::Result<Self> {
        let ini = ini::Ini::load_from_str_noescape(contents)
            .map_err(|_| anyhow::anyhow!("Failed to parse ini file"))?;

        let (timeout, mut default_os) = if let Some(freeldr) = ini.section(Some("FREELOADER")) {
            let timeout = freeldr
                .get("TimeOut")
                .map(|s| s.parse::<u64>().unwrap_or(10))
                .unwrap_or(10);

            let default_os = freeldr.get("DefaultOS").map(|s| s.to_string());

            (timeout, default_os)
        } else {
            (10, None)
        };

        let mut operating_systems = Vec::new();

        if let Some(os_list) = ini.section(Some("Operating Systems")) {
            for (section_name, display_name) in os_list.iter() {
                if let Some(section) = ini.section(Some(section_name)) {
                    let os = OperatingSystem::parse_from_section(display_name, section)?;

                    if default_os
                        .as_ref()
                        .map(|default_os| section_name == default_os)
                        .unwrap_or(true)
                    {
                        default_os = Some(section_name.to_string());

                        operating_systems.insert(0, os);
                    } else {
                        operating_systems.push(os);
                    }
                }
            }
        }

        let new = Self {
            timeout,
            default_os,
            operating_systems,
        };

        log::debug!("parsed options: {new:?}");

        Ok(new)
    }
}
