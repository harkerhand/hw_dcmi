//! Device of the DCMI.
#[cfg(not(feature = "load_dynamic"))]
use hw_dcmi_sys::bindings as ffi;
use crate::{call_dcmi_function, DCMI};
use crate::enums::UnitType;
use crate::error::{DCMIResult, dcmi_try};
use crate::structs::{ChipInfo, DomainPCIEInfo, HBMInfo, MemoryInfo, PCIEInfo};
#[cfg(feature = "serde")]
use serde_derive::{Serialize, Deserialize};

/// Npu management unit
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Card<'a> {
    #[cfg_attr(not(feature = "load_dynamic"), allow(dead_code))]
    pub(crate) dcmi: &'a DCMI,
    pub(crate) id: u32
}

impl Card<'_> {
    /// Create a new card
    /// 
    /// # Warning
    /// It is your responsibility to ensure that the card ID is valid
    pub fn new_unchecked(dcmi: &DCMI, id: u32) -> Card {
        Card{dcmi, id}
    }
    
    /// Query the ID of this card
    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Card<'_> {
    /// Query number of NPU chip in specific NPU management unit
    ///
    /// # Returns
    /// number of NPU chip
    pub fn get_chip_num(&self) -> DCMIResult<u32> {
        let mut device_num = 0i32;

        call_dcmi_function!(dcmi_get_device_num_in_card, self.dcmi.lib, self.id as i32, &mut device_num);

        Ok(device_num as u32)
    }

    /// Get the (NPU chip list, MCU chip, CPU chip) of the specified NPU management unit
    ///
    /// # Returns
    /// each element of return tuple means:
    /// - Vec<Chip>: NPU chip list
    /// - Option<Chip>: MCU chip, if there is no MCU chip, it will be None
    /// - Option<Chip>: CPU chip, if there is no CPU chip, it will be None
    pub fn get_chips(&self) -> DCMIResult<(Vec<Chip>, Option<Chip>, Option<Chip>)> {
        let mut device_id_max = 0i32;
        let mut mcu_id = 0i32;
        let mut cpu_id = 0i32;

        call_dcmi_function!(dcmi_get_device_id_in_card, self.dcmi.lib, self.id as i32, &mut device_id_max, &mut mcu_id, &mut cpu_id);
        
        let npu_chips = (0..device_id_max).into_iter()
            .map(|id| Chip{card: &self, id: id as u32})
            .collect::<Vec<_>>();
        let mcu_chip = if mcu_id != -1 {
            Some(Chip{card: &self, id: mcu_id as u32})
        } else {
            None
        };
        let cpu_chip = if cpu_id != -1 {
            Some(Chip{card: &self, id: cpu_id as u32})
        } else {
            None
        };

        Ok((npu_chips, mcu_chip, cpu_chip))
    }
}

/// Chip of the DCMI
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Chip<'a, 'b> where 'b: 'a {
    pub(crate) id: u32,
    pub(crate) card: &'a Card<'b>,
}

impl<'a, 'b> Chip<'a, 'b> where 'b: 'a {
    /// Create a new chip
    /// 
    /// # Warning
    /// It is your responsibility to ensure that the chip ID is valid
    pub fn new_unchecked(card: &'a Card<'b>, chip_id: u32) -> Self {
        Chip {
            id: chip_id,
            card,
        }
    }
    
    /// Query the ID of this chip
    pub fn id(&self) -> u32 {
        self.id
    }
    
    /// Query the card of this chip
    /// 
    /// # Returns
    /// card
    pub fn card(&self) -> &Card {
        self.card
    }
}

impl Chip<'_, '_> {
    pub fn get_type(&self) -> DCMIResult<UnitType> {
        let mut unit_type = unsafe { std::mem::zeroed() };

        call_dcmi_function!(dcmi_get_device_type, self.card.dcmi.lib, self.card.id as i32, self.id as i32, &mut unit_type);

        Ok(unit_type.into())
    }
    
    /// Query the chip information
    /// 
    /// # Returns
    /// chip information
    pub fn get_info(&self) -> DCMIResult<ChipInfo> {
        let mut chip_info = unsafe { std::mem::zeroed() };

        call_dcmi_function!(dcmi_get_device_chip_info, self.card.dcmi.lib, self.card.id as i32, self.id as i32, &mut chip_info);

        Ok(chip_info.into())
    }
    
    /// Query the PCIE information
    /// 
    /// # Warning
    /// Only NPU chip has PCIE information
    /// 
    /// # Returns
    /// PCIE information
    pub fn get_pcie_info(&self) -> DCMIResult<PCIEInfo> {
        let mut pcie_info = unsafe { std::mem::zeroed() };

        call_dcmi_function!(dcmi_get_device_pcie_info, self.card.dcmi.lib, self.card.id as i32, self.id as i32, &mut pcie_info);

        Ok(pcie_info.into())
    }
    
    /// Query the PCIE information with domain
    /// 
    /// # Warning
    /// Only NPU chip has PCIE information
    /// 
    /// # Returns
    /// PCIE information with domain
    pub fn get_domain_pcie_info(&self) -> DCMIResult<DomainPCIEInfo> {
        let mut pcie_info = unsafe { std::mem::zeroed() };

        call_dcmi_function!(dcmi_get_device_pcie_info_v2, self.card.dcmi.lib, self.card.id as i32, self.id as i32, &mut pcie_info);

        Ok(pcie_info.into())
    }
    
    /// Query the memory information
    /// 
    /// # Warning
    /// Only some of NPU chip has memory information
    /// 
    /// # Returns
    /// memory information
    pub fn get_memory_info(&self) -> DCMIResult<MemoryInfo> {
        let mut memory_info = unsafe { std::mem::zeroed() };
        println!("query memory with card id: {}, chip id: {}", self.card.id, self.id);
        call_dcmi_function!(dcmi_get_device_memory_info_v3, self.card.dcmi.lib, self.card.id as i32, self.id as i32, &mut memory_info);

        Ok(memory_info.into())
    }
    
    /// Query the HBM information
    /// 
    /// # Warning
    /// Only some of NPU chip has HBM information
    /// 
    /// # Returns
    /// HBM information
    pub fn get_hbm_info(&self) -> DCMIResult<HBMInfo> {
        let mut hbm_info = unsafe { std::mem::zeroed() };

        call_dcmi_function!(dcmi_get_device_hbm_info, self.card.dcmi.lib, self.card.id as i32, self.id as i32, &mut hbm_info);

        Ok(hbm_info.into())
    }
}
