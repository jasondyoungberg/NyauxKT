use crate::mem::virt::VMMFlags;

use super::super::virt::cur_pagemap;
pub struct Bump {
    ptr: *mut u8
}
pub static mut BUM: Option<Bump> = None;
impl Bump {
    pub fn new() {
        let q = 
        unsafe {
            cur_pagemap.as_mut().unwrap().vmm_region_alloc(50000000, VMMFlags::KTPRESENT.bits() | VMMFlags::KTWRITEALLOWED.bits())
        }.unwrap();
        unsafe {
            BUM = Some(Bump {ptr: q})
        }
        
    }
    pub fn alloc(&mut self, size: usize) -> *mut u8{
        let old = self.ptr;
        unsafe {
            
            self.ptr = self.ptr.add(super::align_up(size, 8)) as *mut u8;
        }
        return old;
        
    }
}