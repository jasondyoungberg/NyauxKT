use bitflags::{bitflags, Flags};
use limine::{memory_map::EntryType, request::KernelAddressRequest};

use crate::{mem::phys::MEMMAP, println};

use super::phys::{HDDM_OFFSET, PMM};

bitflags! {
    pub struct VMMFlags: u64 
    {
        const KTEXECUTABLEDISABLE = 1 << 63;
        const KTPRESENT = 1;
        const KTWRITEALLOWED = 1 << 1;
        const KTUSERMODE = 1 << 2;
        const KTWRITETHROUGH = 1 << 3;
        const KTCACHEDISABLE = 1 << 4;
    }
}
struct VMMRegion
{
    base: u64,
    length: u64,
    flags: u64,
    next: Option<*mut VMMRegion>
}
pub struct PageMap
{
    head: Option<*mut VMMRegion>,
    rootpagetable: *mut u64
}
#[used]
#[link_section = ".requests"]
pub static ADDR: KernelAddressRequest = KernelAddressRequest::new();
pub static mut cur_pagemap: Option<PageMap> = None;
extern "C"
{
    static THE_REAL: u8;
}
impl PageMap
{
    fn get_next_table(table: *mut u64, index: u64, allocate: bool, flags: Option<u64>) -> *mut u64
    {
        unsafe {
            if table.is_null() || (table as usize) % core::mem::align_of::<u64>() != 0 {
                panic!("fucking helllll");
            }
            if (*table.offset(index as isize)) & VMMFlags::KTPRESENT.bits() == 0
            {
                if (allocate == true)
                {
                    let mut new_table = PMM.alloc().unwrap() as *mut u64;
                    new_table = (new_table as u64 + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u64;
                    new_table.write_bytes(0, 4096 / 8); // memset it
                    // read docs for more information on why its 4096 /8 and not 4096
                    // https://doc.rust-lang.org/std/ptr/fn.write_bytes.html
                    new_table = (new_table as u64 - HDDM_OFFSET.get_response().unwrap().offset()) as *mut u64;
                    let mut entry = 0;
                    entry = new_table as u64;
                    if let Some(q) = flags
                    {
                        entry = entry | q;
                    }
                    
                    (*table.offset(index as isize)) = entry;
                    return ((*table.offset(index as isize)) & 0x000FFFFFFFFFF000) as *mut u64
                }
                else 
                {
                    return 0 as *mut u64;
                }
            }
            else {
                return ((*table.offset(index as isize)) & 0x000FFFFFFFFFF000) as *mut u64
            }
        }
        
    }
    fn map(&mut self, from_virt: u64, to_phys: u64, flags: u64) -> Result<(), &'static str>
    {
        let lvl4table_index = (from_virt >> 39) & 0x1FF;
        let lvl3table_index = (from_virt >> 30) & 0x1FF;
        let lvl2table_index = (from_virt >> 21) & 0x1FF;
        let lvl1table_index = (from_virt >> 12) & 0x1FF;
        let mut cur_table = (self.rootpagetable as u64 + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u64;
        cur_table = PageMap::get_next_table(cur_table, lvl4table_index, true, Some(flags));
        cur_table = PageMap::get_next_table((cur_table as u64 + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u64, lvl3table_index, true, Some(flags));
        cur_table = PageMap::get_next_table((cur_table as u64 + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u64, lvl2table_index, true, Some(flags));
        let mut entry = 0;
        entry = to_phys as u64;
        entry = entry | flags;
        unsafe {
            (*((cur_table as u64 + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u64).offset(lvl1table_index as isize)) = entry;
        }
        

        Ok(())
    }
    fn get_phys_from_entry(pte: u64) -> u64
    {
        return pte & 0x0007FFFFFFFFF000
    }
    fn unmap(&mut self, from_virt: u64, flags: u64) -> Result<(), &'static str>
    {
        let lvl4table_index = (from_virt >> 39) & 0x1FF;
        let lvl3table_index = (from_virt >> 30) & 0x1FF;
        let lvl2table_index = (from_virt >> 21) & 0x1FF;
        let lvl1table_index = (from_virt >> 12) & 0x1FF;
        let mut cur_table = (self.rootpagetable as u64 + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u64;
        cur_table = PageMap::get_next_table(cur_table, lvl4table_index, false, Some(flags));
        if cur_table as u64 == 0
        {
            return Ok(())
        }
        cur_table = PageMap::get_next_table((cur_table as u64 + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u64, lvl3table_index, false, Some(flags));
        if cur_table as u64 == 0
        {
            return Ok(())
        }
        cur_table = PageMap::get_next_table((cur_table as u64 + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u64, lvl2table_index, false, Some(flags));
        if cur_table as u64 == 0
        {
            return Ok(())
        }
        let mut entry = 0;
        entry = 0;
        unsafe {
            (*((cur_table as u64 + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u64).offset(lvl1table_index as isize)) = entry;
            core::arch::asm!("invlpg {x}", x = out(reg) _);
        }
        

        Ok(())
    }
    fn virt_to_phys(&mut self, from_virt: u64) -> Result<u64, &'static str>
    {
        let lvl4table_index = (from_virt >> 39) & 0x1FF;
        let lvl3table_index = (from_virt >> 30) & 0x1FF;
        let lvl2table_index = (from_virt >> 21) & 0x1FF;
        let lvl1table_index = (from_virt >> 12) & 0x1FF;
        let mut cur_table = (self.rootpagetable as u64 + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u64;
        cur_table = PageMap::get_next_table(cur_table, lvl4table_index, false, None);
        if cur_table as u64 == 0
        {
            return Err("Nope")
        }
        cur_table = PageMap::get_next_table((cur_table as u64 + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u64, lvl3table_index, false, None);
        if cur_table as u64 == 0
        {
            return Err("nope")
        }
        cur_table = PageMap::get_next_table((cur_table as u64 + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u64, lvl2table_index, false, None);
        if cur_table as u64 == 0
        {
            return Err("nope")
        }
        
        unsafe {
            if *((cur_table as u64 + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u64).offset(lvl1table_index as isize) != 0
            {
                return Ok(PageMap::get_phys_from_entry(*((cur_table as u64 + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u64).offset(lvl1table_index as isize)));
            }
            else {return Err("nope")};
           
        }
    }
    fn switch_to(&self)
    {
        unsafe {core::arch::asm!(
            "mov cr3, {}",
            in(reg) self.rootpagetable as u64,
        );}
    }
    pub fn new_inital()
    {
        let mut q = PageMap {
            head: None,
            rootpagetable: unsafe {PMM.alloc().unwrap()} as *mut u64,
        };
        unsafe {q.rootpagetable.write_bytes(0, 4096 / 8)};
        let size_pages = unsafe {&THE_REAL as *const _ as usize} / 4096;
        println!("Size of kernel in pages: {}", size_pages);
        println!("Kernel Location: phys: {:#x} virt: {:#x}", ADDR.get_response().unwrap().physical_base(), ADDR.get_response().unwrap().virtual_base());
        for i in 0..=size_pages
        {
            // println!("mapping virt {:#x} to phys {:#x}", ADDR.get_response().unwrap().virtual_base() + (i * 4096) as u64, ADDR.get_response().unwrap().physical_base() + (i * 4096) as u64);
            q.map(
                ADDR.get_response().unwrap().virtual_base() + (i * 4096) as u64, 
                ADDR.get_response().unwrap().physical_base() + (i * 4096) as u64, 
                VMMFlags::KTPRESENT.bits() | VMMFlags::KTWRITEALLOWED.bits()).unwrap();
            
        }
        println!("mapped the kernel i supposeeee");
        println!("um");
        let entries = MEMMAP.get_response().unwrap().entries();
        let mut hhdm_pages = 0;
        
        for i in entries.iter()
        {
            match i.entry_type
            {
                EntryType::ACPI_NVS | EntryType::ACPI_RECLAIMABLE
                | EntryType::USABLE | EntryType::BOOTLOADER_RECLAIMABLE
                | EntryType::FRAMEBUFFER | EntryType::KERNEL_AND_MODULES =>
                {
                    let page_amount = super::phys::align_up(i.length as usize, 4096) / 4096;
                    for e in 0..page_amount
                    {
                        q.map(
                            HDDM_OFFSET.get_response().unwrap().offset() + (e * 4096) as u64,
                            i.base + (e * 4096) as u64,
                            VMMFlags::KTPRESENT.bits() | VMMFlags::KTWRITEALLOWED.bits()
                        ).unwrap()
                    }
                }
                _ => {

                }
            }
        }
        q.switch_to();
    }
}