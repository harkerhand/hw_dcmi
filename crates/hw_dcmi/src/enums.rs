//! Wrapped enums for the DCMI peripheral

use hw_dcmi_sys::bindings as ffi;

#[cfg(feature = "serde")]
use serde_derive::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UnitType {
    NPU,
    MCU,
    CPU,
    Invalid,
}

impl From<ffi::dcmi_unit_type> for UnitType {
    fn from(unit: ffi::dcmi_unit_type) -> Self {
        match unit {
            ffi::dcmi_unit_type_NPU_TYPE => UnitType::NPU,
            ffi::dcmi_unit_type_MCU_TYPE => UnitType::MCU,
            ffi::dcmi_unit_type_CPU_TYPE => UnitType::CPU,
            ffi::dcmi_unit_type_INVALID_TYPE => UnitType::Invalid,
            _ => unreachable!("Not mentioned in the reference manual"),
        }
    }
}
