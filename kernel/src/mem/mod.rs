use core::{alloc::GlobalAlloc, ptr::write_bytes};

use phys::KmallocManager;
use virt::{cur_pagemap, VMMFlags};

pub mod gdt;
pub mod phys;
pub mod virt;

pub struct MemoryManager;

unsafe impl GlobalAlloc for MemoryManager {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        match KmallocManager.as_mut().unwrap().alloc(layout.size()) {
            Some(q) => {
                q.write_bytes(0, layout.size());
                return q;
            }
            None => {
                return cur_pagemap
                    .as_mut()
                    .unwrap()
                    .vmm_region_alloc(
                        layout.size() as u64,
                        VMMFlags::KTPRESENT.bits() | VMMFlags::KTWRITEALLOWED.bits(),
                    )
                    .unwrap();
            }
        }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        if layout.size() > 4096 {
            return cur_pagemap.as_mut().unwrap().vmm_region_dealloc(ptr as u64);
        } else {
            return KmallocManager.as_mut().unwrap().free(ptr as u64);
        }
    }
    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        layout: core::alloc::Layout,
        new_size: usize,
    ) -> *mut u8 {
        if layout.size() > 4096 {
            let new = cur_pagemap
                .as_mut()
                .unwrap()
                .vmm_region_alloc(
                    new_size as u64,
                    VMMFlags::KTPRESENT.bits() | VMMFlags::KTWRITEALLOWED.bits(),
                )
                .unwrap();
            ptr.copy_to(new, layout.size());
            cur_pagemap.as_mut().unwrap().vmm_region_dealloc(ptr as u64);
            return new as *mut u8;
        } else {
            let new = KmallocManager.as_mut().unwrap().alloc(new_size).unwrap();
            ptr.copy_to(new, layout.size());
            KmallocManager.as_mut().unwrap().free(ptr as u64);
            return new;
        }
    }
    unsafe fn alloc_zeroed(&self, layout: core::alloc::Layout) -> *mut u8 {
        if layout.size() > 4096 {
            return cur_pagemap
                .as_mut()
                .unwrap()
                .vmm_region_alloc(
                    layout.size() as u64,
                    VMMFlags::KTPRESENT.bits() | VMMFlags::KTWRITEALLOWED.bits(),
                )
                .unwrap();
        } else {
            let q = KmallocManager
                .as_mut()
                .unwrap()
                .alloc(layout.size())
                .unwrap();
            write_bytes(q, 0, 4096 / 8);
            return q;
        }
    }
}
#[cfg_attr(not(test), no_main, no_std)]
#[cfg_attr(target_os = "none", global_allocator)]
pub static global: MemoryManager = MemoryManager;
