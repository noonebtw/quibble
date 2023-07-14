use alloc::{
    string::{String, ToString},
    vec::Vec,
};

pub mod ffi {
    use core::ptr::null_mut;

    use alloc::{ffi::CString, vec::Vec};

    #[repr(C)]
    pub struct OperatingSystem {
        display_name: *const u8,
        system_path: *const u8,
        options: *const u8,
    }

    impl OperatingSystem {
        fn from_os(os: &super::OperatingSystem) -> Self {
            let display_name = CString::new(os.display_name.clone())
                .expect("null byte in display_name.")
                .into_raw()
                .cast();
            let system_path = CString::new(os.system_path.clone())
                .expect("null byte in system_path.")
                .into_raw()
                .cast();

            let options = os
                .options
                .as_ref()
                .map(|options| {
                    CString::new(options.clone())
                        .expect("null byte in options")
                        .into_raw()
                })
                .unwrap_or(null_mut())
                .cast();

            Self {
                display_name,
                system_path,
                options,
            }
        }

        #[no_mangle]
        pub extern "C" fn operating_system_destroy(self) {
            unsafe {
                if !self.options.is_null() {
                    _ = CString::from_raw(self.options as *mut _);
                }
                _ = CString::from_raw(self.display_name as *mut _);
                _ = CString::from_raw(self.system_path as *mut _);
            }
        }
    }

    #[repr(C)]
    pub struct QuibbleOptions {
        timeout: u64,
        default_os: *const u8,
        operating_systems: *const OperatingSystem,
        operating_systems_len: usize,
        operating_systems_capacity: usize,
    }

    impl QuibbleOptions {
        fn from_options(options: super::Options) -> Self {
            let default_os = options
                .default_os
                .as_ref()
                .map(|options| {
                    CString::new(options.clone())
                        .expect("null byte in default os name")
                        .into_raw()
                })
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
        pub extern "C" fn quibble_options_destroy(self) {
            unsafe {
                if !self.default_os.is_null() {
                    _ = CString::from_raw(self.default_os as *mut _);
                }
                _ = Vec::from_raw_parts(
                    self.operating_systems.cast_mut(),
                    self.operating_systems_len,
                    self.operating_systems_capacity,
                );
            }
        }

        #[no_mangle]
        pub extern "C" fn parse_quibble_options(data: *const u8, len: usize) -> Self {
            let contents =
                core::str::from_utf8(unsafe { core::slice::from_raw_parts(data.cast(), len) })
                    .expect("ini file contents were not proper utf8.");
            let options =
                super::Options::parse_from_bytes(contents).expect("failed to parse ini file.");

            Self::from_options(options)
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
