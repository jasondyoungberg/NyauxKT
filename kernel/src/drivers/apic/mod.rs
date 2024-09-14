use hashbrown::HashMap;
use uacpi::{
    table_find_by_signature, MadtIoapic, MadtIrqSourceOverride, MADT_SIGNATURE
};
use crate::println;
use crate::mem::phys::HDDM_OFFSET;
extern crate alloc;
use alloc::vec::Vec;
struct ioapic {
    handles: (usize, usize), // base gsi to max gsi this ioapic handles
    sourceoveride: HashMap<usize, MadtIrqSourceOverride>, // if theres a irq source override for a gsi, it will be in this hashmap
    // you can check for it and get the REAL gsi from the value
    id: u8, // ioapic id
    addr: u32, // addr of ioapic
}
impl ioapic {
    fn select_reg(&self, reg: u8) {
        unsafe {
            
            *((self.addr as u64 + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u32) = reg as u32;
        }
    }
    fn write_toreg(&self, val: u32) {
        unsafe {
            
            *((self.addr as u64 + 0x10 + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u32) = val;
        }
    }
    fn read_reg(&self) -> u32 {
        unsafe {
            
            return *((self.addr as u64 + 0x10 + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u32);
        }
    }
    fn get_maxgsi(&self) -> u8 {
        self.select_reg(0x1);
        ((self.read_reg() >> 16) & 0xff) as u8
    }
}
struct SystemAPIC {
    apics: Vec<ioapic>
}
static mut apicdriv: Option<SystemAPIC> = None;

pub fn apic_init() {
    let mut table: *mut uacpi::Madt =  table_find_by_signature(MADT_SIGNATURE).unwrap().get_virt_addr() as *mut uacpi::Madt;
    println!("got table");
}