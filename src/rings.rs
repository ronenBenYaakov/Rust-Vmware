#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TxDescriptor {
    pub addr: u64,
    pub length: u16,
    pub cso: u8,
    pub cmd: u8,
    pub status: u8,
    pub css: u8,
    pub special: u16,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct RxDescriptor {
    pub addr: u64,
    pub length: u16,
    pub checksum: u16,
    pub status: u8,
    pub errors: u8,
    pub special: u16,
}


const NUM_TX_DESC: usize = 64;
const NUM_RX_DESC: usize = 64;

static mut TX_DESC_RING: [TxDescriptor; NUM_TX_DESC] = [TxDescriptor { addr: 0, length: 0, cso: 0, cmd: 0, status: 0, css: 0, special: 0 }; NUM_TX_DESC];
static mut RX_DESC_RING: [RxDescriptor; NUM_RX_DESC] = [RxDescriptor { addr: 0, length: 0, checksum: 0, status: 0, errors: 0, special: 0 }; NUM_RX_DESC];


use core::ptr::write_volatile;
use core::sync::atomic::{AtomicUsize, Ordering};

static MMIO_BASE: AtomicUsize = AtomicUsize::new(0);

pub fn set_mmio_base(addr: usize) {
    MMIO_BASE.store(addr, Ordering::SeqCst);
}


pub fn write_register(offset: u32, value: u32) {
    let base = MMIO_BASE.load(Ordering::SeqCst) as *mut u32;
    unsafe {
        write_volatile(base.add((offset / 4) as usize), value);
    }
}

use core::ptr::read_volatile;

pub fn read_register(offset: u32) -> u32 {
    let base = MMIO_BASE.load(Ordering::SeqCst) as *const u32;
    unsafe { read_volatile(base.add((offset / 4) as usize)) }
}

