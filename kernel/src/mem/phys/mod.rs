use core::alloc::GlobalAlloc;
use core::alloc::Layout;

use limine::{
    memory_map::EntryType,
    request::{HhdmRequest, MemoryMapRequest},
};
use owo_colors::OwoColorize;

use crate::print;
use crate::println;
use crate::utils::KTError;

#[derive(Debug)]
pub struct KTNode {
    next: Option<*mut KTNode>,
}

pub struct PhysicalAllocator {
    head: Option<*mut KTNode>,
}
#[repr(C)]
#[derive(PartialEq)]
struct slab_header {
    size: usize,
    next_slab: Option<*mut slab_header>,
    freelist: Option<*mut KTNode>,
}
#[derive(PartialEq)]
struct cache {
    slabs: Option<*mut slab_header>,
    size: usize,
}
pub struct kmalloc_manager {
    array: [cache; 7],
}
impl kmalloc_manager {
    fn init() -> Self {
        let mut cache1 = cache::init(16);
        let mut cache2 = cache::init(32);
        let mut cache3 = cache::init(64);
        let mut cache4 = cache::init(128);
        let mut cache5 = cache::init(256);
        let mut cache6 = cache::init(512);
        let mut cache7 = cache::init(1024);
        Self {
            array: [cache1, cache2, cache3, cache4, cache5, cache6, cache7],
        }
    }
    pub fn free(&mut self, addr: u64) {
        if addr == 0 {
            return;
        }
        let mut h = (addr & !0xFFF) as *mut slab_header;
        let mut rightcache = None;
        'outer: for i in self.array.iter_mut() {
            unsafe {
                if (i.size == (*h).size) {
                    rightcache = Some(i);
                    break 'outer;
                }
            }
        }
        if rightcache == None {
            return;
        }
        let mut new = addr as *mut KTNode;

        unsafe { new.write_bytes(0, 1) };
        unsafe {
            (*new).next = (*h).freelist;
            (*h).freelist = Some(new);
            let mut prev = None;
            let mut shit = rightcache.unwrap().slabs;
            while (shit != None) {
                if ((*shit.unwrap()) == *h) {
                    return;
                } else {
                    prev = shit;
                    shit = (*shit.unwrap()).next_slab;
                }
            }
            (*prev.unwrap()).next_slab = Some(h);
            return;
        }
    }
    pub fn alloc(&mut self, size: usize) -> Option<*mut u8> {
        let a = size.next_power_of_two();
        for i in self.array.iter_mut() {
            if i.size >= a {
                return i.slab_allocsearch();
            }
        }
        None
    }
}
#[used]
#[link_section = ".requests"]
pub static HDDM_OFFSET: HhdmRequest = HhdmRequest::new();

#[used]
#[link_section = ".requests"]
pub static mut MEMMAP: MemoryMapRequest = MemoryMapRequest::new();
/// stolen troll
pub fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
pub fn align_down(addr: usize, align: usize) -> usize {
    addr & !(align - 1)
}
pub static mut PMM: PhysicalAllocator = PhysicalAllocator { head: None };
pub static mut kmalloc_manager: Option<kmalloc_manager> = None;

impl PhysicalAllocator {
    pub fn new() -> Result<(), &'static str> {
        println!("{}", "--Memory MAP--".red());
        let entries = unsafe { MEMMAP.get_response().unwrap().entries() };

        let mut new = PhysicalAllocator { head: None };
        let mut last = None;
        for i in entries.iter() {
            match i.entry_type {
                EntryType::USABLE => {
                    let page_amount = align_up(i.length as usize, 4096) / 4096;
                    for e in 0..page_amount {
                        unsafe {
                            let node: *mut KTNode = ((i.base + (e as u64 * 4096))
                                + HDDM_OFFSET.get_response().unwrap().offset())
                                as *mut KTNode;
                            (*node).next = last;
                            last = Some(node);
                        }
                    }

                    println!(
                        "Created Freelist Node of Base {:#x} and Page Count {}",
                        i.base.yellow(),
                        (align_up(i.length as usize, 4096) / 4096).green()
                    );
                }
                _ => {}
            }
        }
        new.head = last;

        unsafe { PMM = new };
        unsafe { kmalloc_manager = Some(kmalloc_manager::init()) }
        return Ok(());
    }
    pub fn alloc(&mut self) -> Result<*mut u8, KTError> {
        let mut w = self.head.unwrap();
        'outer: loop {
            match unsafe { (*w).next } {
                Some(e) => {
                    self.head = Some(e);
                    return Ok((w as u64 - HDDM_OFFSET.get_response().unwrap().offset()) as *mut u8);
                }
                None => {
                    break 'outer;
                }
            }
        }
        println!("Reached end");
        return Err(KTError::OutOfMemory);
    }
    pub fn dealloc(&mut self, addr: *mut u8) -> Result<(), KTError> {
        let mut w = self.head.unwrap();
        let e = align_down(addr as usize, 4096);
        println!("returning address of aligned addr: {:#x}", e);

        let node: *mut KTNode = addr as *mut KTNode;
        unsafe {
            (*node).next = self.head;
            self.head = Some(node);
        }
        Ok(())
    }
}

impl slab_header {
    fn init(size: usize) -> *mut Self {
        let mut area: *mut u64 = unsafe { PMM.alloc().unwrap() as *mut u64 };
        area = (area as u64 + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u64;
        unsafe { area.write_bytes(0, 4096 / 8) };
        let mut header = (area) as *mut slab_header;

        unsafe {
            header.write_bytes(0, 1);
            (*header).size = size;

            let obj_amount = (4096 - size_of::<slab_header>()) / size;
            let mut start = (header as u64 + size_of::<slab_header>() as u64) as *mut KTNode;
            println!("objection ammount: {obj_amount}");

            (*header).freelist = Some(start);
            start.write_bytes(0, 1);
            (*start).next = None;
            let mut prev = start;

            println!("prev addr is {:#x}", prev as u64);

            for i in 1..obj_amount {
                let mut new = (start as u64 + (i as u64 * size as u64)) as *mut KTNode;
                new.write(KTNode { next: None });
                (*new).next = None;

                (*prev).next = Some(new);

                prev = new;
            }

            (*prev).next = None;
        }
        return header;
    }
}
impl cache {
    fn init(size: usize) -> Self {
        let mut new = slab_header::init(size);
        println!("Created Cache of size: {size}");
        Self {
            size: size,
            slabs: Some(new),
        }
    }
    fn slab_allocsearch(&mut self) -> Option<*mut u8> {
        let mut h = self.slabs;
        'outer: while h.is_none() == false {
            unsafe {
                if (*h.unwrap()).freelist.is_some() {
                    let mut new = (*h.unwrap()).freelist.unwrap();
                    println!("new struct: {:?}", *new);
                    (*h.unwrap()).freelist = (*new).next;
                    return Some(new as *mut u8);
                } else {
                    if (*h.unwrap()).next_slab.is_none() {
                        break 'outer;
                    }
                    h = (*h.unwrap()).next_slab;
                }
            }
        }
        // make new slab for cache since theres no more space
        let mut new = slab_header::init(self.size);
        unsafe {
            (*h.unwrap()).next_slab = Some(new);
            let mut o = (*new).freelist.unwrap();
            (*new).freelist = (*o).next;
            return Some(o as *mut u8);
        }
    }
}
