use crate::DCMI;

#[test]
fn test_get_card_list() {
    let dcmi = DCMI::init().unwrap();
    let card_list = dcmi.get_card_list().unwrap();
    println!("card num: {}, card list: {:?}", card_list.len(), card_list);
}

#[test]
fn test_get_memory_info() {
    let dcmi = DCMI::init().unwrap();
    let card_list = dcmi.get_card_list().unwrap();
    for card in card_list {
        let (chips, mcu_chip, cpu_chip) = card.get_chips().unwrap();
        println!("chips: {:?}, mcu_chip: {:?}, cpu_chip: {:?}", chips, mcu_chip, cpu_chip);
        for chip in chips {
            let memory_info = chip.get_memory_info().unwrap();
            println!("chip memory info: {:?}", memory_info);
        }
    }
}

#[test]
fn test_get_hbm_info() {
    let dcmi = DCMI::init().unwrap();
    let card_list = dcmi.get_card_list().unwrap();
    for card in card_list {
        let (chips, mcu_chip, cpu_chip) = card.get_chips().unwrap();
        println!("chips: {:?}, mcu_chip: {:?}, cpu_chip: {:?}", chips, mcu_chip, cpu_chip);
        for chip in chips {
            let hbm_info = chip.get_hbm_info().unwrap();
            println!("chip hbm info: {:?}", hbm_info);
        }
    }
}
