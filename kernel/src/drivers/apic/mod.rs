use hashbrown::HashMap;
use owo_colors::OwoColorize;
use uacpi::sys::{acpi_entry_hdr, acpi_madt_interrupt_source_override, acpi_madt_ioapic, ACPI_MADT_ENTRY_TYPE_INTERRUPT_SOURCE_OVERRIDE, ACPI_MADT_ENTRY_TYPE_IOAPIC};
use uacpi::{
    table_find_by_signature, MadtIoapic, MadtIrqSourceOverride, MADT_SIGNATURE
};
use crate::cpu::{lapic, CPU};
use crate::cpu::lapic::LAPIC;
use crate::println;
use crate::mem::phys::HDDM_OFFSET;
extern crate alloc;
use alloc::vec::Vec;
#[derive(Debug)]
struct ioapic {
    handles: (usize, usize), // base gsi to max gsi this ioapic handles
    
    // you can check for it and get the REAL gsi from the value
    id: u8, // ioapic id
    addr: u32, // addr of ioapic
}
impl ioapic {
    fn new(gsi_base: usize, id: u8, addr: u32) -> Self {
        Self {
            handles: (gsi_base, 0),
            
            id: id,
            addr: addr
        }
    }
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
    fn get_maxgsi(&self) -> u32 {
        self.select_reg(0x1);
        ((self.read_reg() >> 16) & 0xff) as u32
    }
    
}
struct SystemAPIC {
    apics: Vec<ioapic>,
    sourceoveride: HashMap<usize, MadtIrqSourceOverride>, // if theres a irq source override for a gsi, it will be in this hashmap
}
impl SystemAPIC {
    pub fn route_irq(&self, irq: u8, lapic_id: u32, vec: u8) {
        let mut real_gsi = irq as u32;
        let mut flags = 1 << 2 | 1;
        if self.sourceoveride.contains_key(&(irq as usize)) {
            flags = self.sourceoveride.get(&(irq as usize)).unwrap().flags;
            real_gsi = self.sourceoveride.get(&(irq as usize)).unwrap().gsi as u32;
        }
        let mut okay: u64 = 0;
        okay |= (lapic_id as u64) << 56;
        okay |= (flags as u64)<< 13;
        okay |= (vec as u64);
        for i in self.apics.iter() {
            if (i.handles.0..=i.handles.1).contains(&(real_gsi as usize)) {
                i.select_reg((0x10 + ((real_gsi - i.handles.0 as u32) * 0x02)) as u8);
                i.write_toreg(okay as u32);
                i.select_reg((0x10 + ((real_gsi - i.handles.0 as u32) * 0x02) + 1) as u8);
                i.write_toreg((okay >> 32) as u32);
            }
        }

    }
}
static mut apicdriv: Option<SystemAPIC> = None;

pub fn apic_init() {
    let mut table: *mut uacpi::Madt =  table_find_by_signature(MADT_SIGNATURE).unwrap().get_virt_addr() as *mut uacpi::Madt;
    unsafe {apicdriv = Some(SystemAPIC {apics: Vec::new(), sourceoveride: HashMap::new()})};
    println!("got table");
    unsafe {
        let length_of_entries = (*table).hdr.length as usize - size_of_val(&*table);
        println!("{}: LENGTH OF ENTRIES IN MADT {}", "APIC".bright_red(), length_of_entries);
        let mut offset = 0;
        while offset < length_of_entries {
            let entry_hdr = ((*table).entries.as_mut_ptr().add(offset / size_of::<acpi_entry_hdr>()));
            println!("entry with type {}", (*entry_hdr).type_);
            match (*entry_hdr).type_ as u32 {
                ACPI_MADT_ENTRY_TYPE_INTERRUPT_SOURCE_OVERRIDE => {
                    println!("{}: feet is good vibe is good", "APIC".bright_red());
                    let real = *(entry_hdr as *mut MadtIrqSourceOverride);
                    apicdriv.as_mut().unwrap().sourceoveride.insert(real.source as usize, real);
                },
                ACPI_MADT_ENTRY_TYPE_IOAPIC => {
                    println!("{}: found IOAPIC", "APIC".bright_red());
                    let real = *(entry_hdr as *mut acpi_madt_ioapic);
                    let q = real.address;
                    let h = real.id;
                    let hnm = real.gsi_base;
                    
                    let mut qq = ioapic::new(hnm as usize, h, q);
                    qq.handles.1 = qq.get_maxgsi() as usize;
                    println!("INFO of ioapic: {:?}", qq);
                    apicdriv.as_mut().unwrap().apics.push(qq);
                    

                },
                _ => {
                    
                }
            }
            offset += (*entry_hdr).length as usize;
            println!("offset is now {offset}");
        }
        println!("{:?}", apicdriv.as_ref().unwrap().apics);
        
        
        // apicdriv.as_mut().unwrap().route_irq(1, CPU::read_lapic_id(CPU::get_lapic_addr()), 47);
        // unsafe {crate::utils::read_from_portu8(0x60)};
    }
}