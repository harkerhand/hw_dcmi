#[warn(missing_docs)]

use hw_dcmi_sys::bindings as ffi;
use static_assertions::assert_impl_all;
use crate::device::Card;
use crate::error::{dcmi_try, DCMIResult};

pub mod error;
pub mod enums;
pub mod device;
pub mod structs;
#[cfg(test)]
mod test;

#[cfg(not(feature = "load_dynamic"))]
#[derive(Debug)]
pub struct DCMI;
#[cfg(feature = "load_dynamic")]
pub struct DCMI {
    pub(crate) lib: ffi::dcmi,
}

#[cfg(feature = "load_dynamic")]
impl std::fmt::Debug for DCMI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DCMI")
    }
}

assert_impl_all!(DCMI: Send, Sync);

#[macro_export]
macro_rules! call_dcmi_function {
    ($func_name:ident, $dcmi:expr, $($arg:expr),*) => {
        dcmi_try(
            unsafe {
                #[cfg(feature = "load_dynamic")]
                {
                    $dcmi.$func_name($($arg),*)
                }
                #[cfg(not(feature = "load_dynamic"))]
                {
                    ffi::$func_name($($arg),*)
                }
            }
        )?
    };
}

impl DCMI {
    /// Initialize the DCMI
    /// 
    /// As documented in the reference manual, the DCMI must be initialized before use.
    pub fn init() -> DCMIResult<Self> {
        #[cfg(feature = "load_dynamic")]
        let lib = {
            let hw_dcmi_path = std::env::var("HW_DCMI_PATH")
                .unwrap_or_else(|_| "/usr/local/dcmi".to_string());
            let lib_path = format!("{}/libdcmi.so", hw_dcmi_path);
            unsafe {
                ffi::dcmi::new(lib_path)
            }.expect("Failed to load DCMI library dynamically")
        };
        
        let res = unsafe {
            #[cfg(feature = "load_dynamic")]
            { lib.dcmi_init() }
            
            #[cfg(not(feature = "load_dynamic"))]
            { ffi::dcmi_init() }
        };
        
        dcmi_try(res)?;
        
        let dcmi = DCMI {
            #[cfg(feature = "load_dynamic")]
            lib: lib,
        };
        Ok(dcmi)
    }

    /// Get the DCMI version
    /// 
    /// # Returns
    /// DCMI version
    pub fn get_dcmi_version(&self) -> DCMIResult<String> {
        let mut dcmi_ver = [0u8; 16]; // 分配一个 16 字节的数组，用于存储 C 字符串
        let len = dcmi_ver.len() as u32;

        // 调用 C 函数
        call_dcmi_function!(dcmi_get_dcmi_version, self.lib, dcmi_ver.as_mut_ptr() as *mut ::std::os::raw::c_char, len);
        
        Ok(std::str::from_utf8(&dcmi_ver)?.into())
    }
    
    /// Get the driver version
    /// 
    /// # Returns
    /// driver version
    pub fn get_driver_version(&self) -> DCMIResult<String> {
        let mut driver_ver = [0u8; 64];
        let len = driver_ver.len() as u32;
        
        call_dcmi_function!(dcmi_get_driver_version, self.lib, driver_ver.as_mut_ptr() as *mut ::std::os::raw::c_char, len);
        
        Ok(std::str::from_utf8(&driver_ver)?.into())
    }
    
    /// Query target device driver version
    /// 
    /// # Parameters
    /// - card_id: Specify the NPU management unit ID, and obtain the currently supported IDs through the `get_card_list`
    /// - device_id: Specify the device ID, and obtain the supported IDs through the `get_device_id_in_card`
    /// 
    /// # Returns
    /// driver version
    #[deprecated = "As mentioned in Huawei document, this function will delete later, Use get_driver_version instead"]
    pub fn get_version(&self, card_id: u32, chip_id: u32) -> DCMIResult<String> {
        let mut driver_ver = [0u8; 64];
        let len = driver_ver.len() as u32;
        let mut ver_len = 0i32;

        call_dcmi_function!(
            dcmi_get_version, self.lib, 
            card_id as std::os::raw::c_int,
            chip_id as std::os::raw::c_int,
            driver_ver.as_mut_ptr() as *mut ::std::os::raw::c_char, 
            len,
            &mut ver_len
        );

        Ok(std::str::from_utf8(&driver_ver.into_iter().take(ver_len as usize).collect::<Vec<_>>())?.into())
    }
    
    /// Query the number of NPU units and the id of each NPU unit
    /// 
    /// # Returns
    /// NPU management unit ID list
    pub fn get_card_list(&self) -> DCMIResult<Vec<Card>> {
        let mut card_num = 0i32;
        let mut card_list = [-1i32; 64];
        let len = card_list.len() as i32;

        call_dcmi_function!(dcmi_get_card_list, self.lib, &mut card_num, card_list.as_mut_ptr(), len);

        Ok(card_list.into_iter().take(card_num as usize).map(|id| Card{dcmi: &self, id: id as u32}).collect())
    }
}
