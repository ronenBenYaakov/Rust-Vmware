use x86_64::instructions::port::{PortRead, PortWrite, Port};

use crate::println;

const PCI_CONFIG_ADDRESS: u16 = 0xcf8;
const PCI_CONFIG_DATA: u16 = 0xcfc;

//pci adress configuration
fn pci_config_address(bus: u8, device: u8, function: u8, register: u8) -> u32 {
    let enable_bit = 1 << 31;
    let bus_bits = (bus as u32) << 16;
    let device_bits = (device as u32) << 11;
    let function_bits = (function as u32) << 8;
    let register_bits = (register as u32) & 0xFC; // align to 4 bytes

    enable_bit | bus_bits | device_bits | function_bits | register_bits
}

//read 32 bits pci configuration
fn pci_read(bus: u8, device: u8, function: u8, register: u8) -> u32 {
    let address = pci_config_address(bus, device, function, register);

    unsafe {
        let mut config_address_port = Port::new(PCI_CONFIG_ADDRESS);
        let mut config_data_port = Port::new(PCI_CONFIG_DATA);
        
        config_address_port.write(address);
        config_data_port.read()
    }
}

fn pci_device_exists(bus: u8, device: u8, function: u8) -> bool {
    let vendor_id = (pci_read(bus, device, function, 0x00) & 0xFFFF) as u16;
    vendor_id != 0xFFFF
}



pub fn pci_scan() {
    for bus in 0..=255 {
        for device in 0..=31 {
            for function in 0..=7 {
                if pci_device_exists(bus, device, function) {
                    let vendor_id = (pci_read(bus, device, function, 0x00) & 0xFFFF) as u16;
                    let device_id = ((pci_read(bus, device, function, 0x00) >> 16) & 0xFFFF) as u16;

                    println!("PCI Device found at bus {}, device {}, function {} - Vendor ID: {:04x}, Device ID: {:04x}",
                        bus, device, function, vendor_id, device_id);

                }
            }
        }
    }
}

