use core::ptr::write_volatile;

use crate::{pci::pci_read, println};

fn read_vendor_device_id(bus: u8, device: u8, function: u8) -> (u16, u16) {
    let data = pci_read(bus, device, function, 0x00);
    let vendor_id = (data & 0xFFFF) as u16;
    let device_id = ((data >> 16) & 0xFFFF) as u16;
    (vendor_id, device_id)
}

fn read_class_info(bus: u8, device: u8, function: u8) -> (u8, u8, u8) {
    let data = pci_read(bus, device, function, 0x08); 
    let revision_id = (data & 0xFF) as u8;
    let prog_if = ((data >> 8) & 0xFF) as u8;     
    let subclass = ((data >> 16) & 0xFF) as u8;
    let class_code = ((data >> 24) & 0xFF) as u8;
    (class_code, subclass, prog_if)
}

fn read_header_type(bus: u8, device: u8, function: u8) -> u8 {
    let data = pci_read(bus, device, function, 0x0C);
    ((data >> 16) & 0xFF) as u8
}

pub fn print_pci_device_details(bus: u8, device: u8, function: u8) {
    let (vendor, device_id) = read_vendor_device_id(bus, device, function);
    let (class_code, subclass, prog_if) = read_class_info(bus, device, function);
    let header_type = read_header_type(bus, device, function);

    println!(
        "Bus: {}, Device: {}, Function: {}",
        bus, device, function
    );
    println!("Vendor ID: {:04x}, Device ID: {:04x}", vendor, device_id);
    println!("Class Code: {:02x}, Subclass: {:02x}, Prog IF: {:02x}", class_code, subclass, prog_if);
    println!("Header Type: {:02x}", header_type);
}

pub unsafe fn reset_device(bar0_mmio_base: u32) {
    // Assume control register is at offset 0x0000
    let ctrl_reg = (bar0_mmio_base + 0x0000) as *mut u32;

    // For many devices, bit 26 is the Reset bit (e.g., E1000_CTRL_RST)
    const RESET_BIT: u32 = 1 << 26;

    // Set reset bit
    core::ptr::write_volatile(ctrl_reg, RESET_BIT);

    // Delay a bit (could use a busy loop or PIT delay)
    for _ in 0..10000 {
        core::arch::asm!("nop");
    }

    // Clear reset bit or let the device clear it
    // (Depends on device behavior — often it self-clears)
}

