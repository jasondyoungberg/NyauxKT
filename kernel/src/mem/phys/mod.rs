use core::alloc::GlobalAlloc;
use core::alloc::Layout;

use limine::{memory_map::EntryType, request::{HhdmRequest, MemoryMapRequest}};
use owo_colors::OwoColorize;

use crate::print;
use crate::println;
use crate::utils::KTError;

pub struct KTNode
{
    next: Option<*mut KTNode>
}

pub struct PhysicalAllocator
{
    head: Option<*mut KTNode>
}
#[used]
#[link_section = ".requests"]
pub static HDDM_OFFSET: HhdmRequest = HhdmRequest::new();

#[used]
#[link_section = ".requests"]
pub static MEMMAP: MemoryMapRequest = MemoryMapRequest::new();
/// stolen troll
pub fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
pub fn align_down(addr: usize, align: usize) -> usize {
    addr & !(align - 1)
}
pub static mut PMM: PhysicalAllocator = PhysicalAllocator {head: None};
impl PhysicalAllocator
{
    pub fn new() -> Result<(), &'static str>
    {
        println!("{}", "--Memory MAP--".red());
        let entries = MEMMAP.get_response().unwrap().entries();
        
        let mut new = PhysicalAllocator {head : None};
        let mut last = None;
        for i in entries.iter()
        {
            
            
            match i.entry_type
            {
                EntryType::USABLE => {
                    
                    let page_amount = align_up(i.length as usize, 4096) / 4096;
                    for e in 0..page_amount
                    {
                        unsafe {
                            let node: *mut KTNode = ((i.base + (e as u64 * 4096)) + HDDM_OFFSET.get_response().unwrap().offset()) as *mut KTNode;
                            (*node).next = last;
                            last = Some(node);
                        }
                        
                    }
                    
                    println!("Created Freelist Node of Base {:#x} and Page Count {}", i.base.yellow(), (align_up(i.length as usize, 4096) / 4096).green());
                   
                    
                }
                _ => {

                }
            }
            
        }
       ;
        new.head = last;
        unsafe {PMM = new};
        return Ok(())
    }
    pub fn alloc(&mut self) -> Result<*mut u8, KTError>
    {
        let mut w = self.head.unwrap();
        'outer: loop {
            
            
            match unsafe {(*w).next}
            {
                Some(e) => {
                    
                    self.head = Some(e);
                    return Ok((w as u64 - HDDM_OFFSET.get_response().unwrap().offset()) as *mut u8);
                    
                },
                None => {
                    break 'outer;
                }
            }
            
        }
        println!("Reached end");
        return Err(KTError::OutOfMemory);
    }
    pub fn dealloc(&mut self, addr: *mut u8) -> Result<(), KTError>
    {
        let mut w = self.head.unwrap();
        let e = align_down(addr as usize, 4096);
        println!("returning address of aligned addr: {:#x}", e);
        
        let node: *mut KTNode = addr as *mut KTNode;
        unsafe {
            (*node).next = self.head;
            self.head = Some(node);
        }
        Err(KTError::NotImplmented)
    }

}
