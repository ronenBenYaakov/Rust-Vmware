use x86_64::instructions::port::{PortRead, PortWrite, Port};

use crate::{device::print_pci_device_details, println};

const PCI_CONFIG_ADDRESS: u16 = 0xcf8;
const PCI_CONFIG_DATA: u16 = 0xcfc;

//pci adress configuration
pub fn pci_config_address(bus: u8, device: u8, function: u8, register: u8) -> u32 {
    let enable_bit = 1 << 31;
    let bus_bits = (bus as u32) << 16;
    let device_bits = (device as u32) << 11;
    let function_bits = (function as u32) << 8;
    let register_bits = (register as u32) & 0xFC; // align to 4 bytes

    enable_bit | bus_bits | device_bits | function_bits | register_bits
}

//read 32 bits pci configuration
pub fn pci_read(bus: u8, device: u8, function: u8, register: u8) -> u32 {
    let address = pci_config_address(bus, device, function, register);

    unsafe {
        let mut config_address_port = Port::new(PCI_CONFIG_ADDRESS);
        let mut config_data_port = Port::new(PCI_CONFIG_DATA);
        
        config_address_port.write(address);
        config_data_port.read()
    }
}

fn pci_write(bus: u8, device: u8, function: u8, register: u8, value: u32) {
    let address = pci_config_address(bus, device, function, register);

    unsafe {
        let mut config_address_port = Port::new(PCI_CONFIG_ADDRESS);
        let mut config_data_port = Port::new(PCI_CONFIG_DATA);

        config_address_port.write(address);
        config_data_port.write(value);
    }
}


pub fn pci_device_exists(bus: u8, device: u8, function: u8) -> bool {
    let vendor_id = (pci_read(bus, device, function, 0x00) & 0xFFFF) as u16;
    vendor_id != 0xFFFF
}



pub fn pci_scan() {
    for bus in 0..=255 {
        for device in 0..=31 {
            for function in 0..=7 {
                if pci_device_exists(bus, device, function) {
                    initialize_device(bus, device, function);
                }
            }
        }
    }
}

pub fn enable_device(bus: u8, device: u8, function: u8) {
    let cmd = (pci_read(bus, device, function, 0x04) & 0xffff) as u16;
    let cmd_new = cmd| 0x07;

    pci_write(bus, device, function, 0x04, cmd_new as u32);
}

pub fn read_bar0(bus: u8, device: u8, function: u8) -> u32 {
    pci_read(bus, device, function, 0x10)
}

pub fn initialize_device(bus: u8, device: u8, function: u8) {
    enable_device(bus, device, function);
    let bar0 = read_bar0(bus, device, function);
    println!("Device BAR0 address: {:x}", bar0);

    // Further device initialization code here
}

pub fn write_bar0(bus: u8, device: u8, function: u8, offset: u32, value: u32) {
    // Read BAR0 register to get base address
    let bar0 = pci_read(bus, device, function, 0x10);

    if (bar0 & 0x1) == 0 {
        // MMIO - Memory Mapped IO
        let mmio_base = bar0 & 0xFFFFFFF0;
        let reg_addr = (mmio_base + offset) as *mut u32;

        unsafe {
            core::ptr::write_volatile(reg_addr, value);
        }
    } else {
        // Port IO
        let port_base = bar0 & 0xFFFFFFFC;

        unsafe {
            let mut port = Port::<u32>::new(port_base + offset);
            port.write(value);
        }
    }
}