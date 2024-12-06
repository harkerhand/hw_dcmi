//! Wrapped structs for the DCMI peripheral

use std::ffi::CStr;
#[cfg(feature = "serde")]
use serde_derive::{Deserialize, Serialize};

use hw_dcmi_sys::bindings as ffi;

/// Chip information
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChipInfo {
    /// Chip type
    pub chip_type: String,
    /// Chip name
    pub chip_name: String,
    /// Chip version
    pub chip_version: String,
    /// Chip AI core count, for MCU and CPU, this field makes no sense
    pub ai_core_count: u32
}

impl From<ffi::dcmi_chip_info> for ChipInfo {
    fn from(chip_info: ffi::dcmi_chip_info) -> Self {
        ChipInfo {
            chip_type: CStr::from_bytes_until_nul(&chip_info.chip_type).unwrap().to_str().unwrap().into(),
            chip_name: CStr::from_bytes_until_nul(&chip_info.chip_name).unwrap().to_str().unwrap().into(),
            chip_version: CStr::from_bytes_until_nul(&chip_info.chip_ver).unwrap().to_str().unwrap().into(),
            ai_core_count: chip_info.aicore_cnt as u32
        }
    }
}

/// PCIE information
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PCIEInfo {
    /// Device ID
    pub device_id: u32,
    /// Vender ID
    pub vender_id: u32,
    /// Subvender ID
    pub subvender_id: u32,
    /// Subdevice ID
    pub subdevice_id: u32,
    /// BDF device ID
    pub bdf_device_id: u32,
    /// BDF bus ID
    pub bdf_bus_id: u32,
    /// BDF function ID
    pub bdf_func_id: u32,
}

impl From<ffi::dcmi_pcie_info> for PCIEInfo {
    fn from(pcie_info: ffi::dcmi_pcie_info) -> Self {
        PCIEInfo {
            device_id: pcie_info.deviceid as u32,
            vender_id: pcie_info.venderid as u32,
            subvender_id: pcie_info.subvenderid as u32,
            subdevice_id: pcie_info.subdeviceid as u32,
            bdf_device_id: pcie_info.bdf_deviceid as u32,
            bdf_bus_id: pcie_info.bdf_busid as u32,
            bdf_func_id: pcie_info.bdf_funcid as u32,
        }
    }
}

impl From<ffi::dcmi_pcie_info_all> for PCIEInfo {
    fn from(pcie_info: ffi::dcmi_pcie_info_all) -> Self {
        PCIEInfo {
            device_id: pcie_info.deviceid as u32,
            vender_id: pcie_info.venderid as u32,
            subvender_id: pcie_info.subvenderid as u32,
            subdevice_id: pcie_info.subdeviceid as u32,
            bdf_device_id: pcie_info.bdf_deviceid as u32,
            bdf_bus_id: pcie_info.bdf_busid as u32,
            bdf_func_id: pcie_info.bdf_funcid as u32,
        }
    }
}

/// PCIE information with domain
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct DomainPCIEInfo {
    pub pcie_info: PCIEInfo,
    /// Domain
    pub domain: i32,
}

impl From<ffi::dcmi_pcie_info_all> for DomainPCIEInfo {
    fn from(pcie_info: ffi::dcmi_pcie_info_all) -> Self {
        DomainPCIEInfo {
            pcie_info: pcie_info.into(),
            domain: pcie_info.domain as i32,
        }
    }
}

/// Memory information
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemoryInfo {
    /// Memory size, unit: MB
    pub memory_size: u64,
    /// Memory available, unit: MB, free + huge_pages_free * huge_page_size
    pub memory_available: u64,
    /// Frequency
    pub freq: u32,
    /// Huge page size, unit: KB
    pub huge_page_size: u64,
    /// Huge pages total
    pub huge_pages_total: u64,
    /// Huge pages free
    pub huge_pages_free: u64,
    /// Utilization, DDR memory info usages
    pub utilization: u32,
}

impl From<ffi::dcmi_get_memory_info_stru> for MemoryInfo {
    fn from(memory_info: ffi::dcmi_get_memory_info_stru) -> Self {
        MemoryInfo {
            memory_size: memory_info.memory_size as u64,
            memory_available: memory_info.memory_available as u64,
            freq: memory_info.freq as u32,
            huge_page_size: memory_info.hugepagesize as u64,
            huge_pages_total: memory_info.hugepages_total as u64,
            huge_pages_free: memory_info.hugepages_free as u64,
            utilization: memory_info.utiliza as u32,
        }
    }
}

/// HBM information
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HBMInfo {
    /// HBM total size, MB
    pub memory_size: u64,
    /// HBM frequency, MHz
    pub frequency: u32,
    /// HBM memory usage, MB
    pub memory_usage: u64,
    /// HBM temperature
    pub temperature: i32,
    /// HBM bandwidth utilization rate
    pub bandwidth_util_rate: u32,
}

impl From<ffi::dcmi_hbm_info> for HBMInfo {
    fn from(hbm_info: ffi::dcmi_hbm_info) -> Self {
        HBMInfo {
            memory_size: hbm_info.memory_size as u64,
            frequency: hbm_info.freq as u32,
            memory_usage: hbm_info.memory_usage as u64,
            temperature: hbm_info.temp as i32,
            bandwidth_util_rate: hbm_info.bandwith_util_rate as u32,
        }
    }
}
