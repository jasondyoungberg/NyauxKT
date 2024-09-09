

use uacpi::{Hpet, HPET_SIGNATURE};

use crate::{
    acpi::ACPIMANAGER,
    mem::{
        phys::{align_down, align_up, HDDM_OFFSET},
        virt::{cur_pagemap, VMMFlags},
    },
    println,
    utils::rdmsr,
};

use super::CPU;

pub trait LAPIC {
    unsafe fn ksleep(&self, ms: u64);
    fn write_lapic_register(&self, reg: u64, val: u32);
    fn read_lapic_register(&self, reg: u64) -> u32;
    fn send_lapic_eoi(lapic_addr: u64);
    fn init_lapic(&mut self);
}
impl LAPIC for CPU {
    unsafe fn ksleep(&self, ms: u64) {
        let pol_start = *((self.hpet_addr_virt + 0x0f0) as *mut u64);
        let pol_cur = (self.hpet_addr_virt + 0x0f0) as *mut u64;
        while ((*(pol_cur) - pol_start) * self.time_per_tick_hpet < ms * 1000000) {
            core::arch::asm!("nop");
        }
    }
    fn write_lapic_register(&self, reg: u64, val: u32) {
        println!("writing");
        unsafe {
            core::ptr::write_volatile((self.lapic_addr + reg) as *mut u32, val);
        }
    }
    fn read_lapic_register(&self, reg: u64) -> u32 {
        println!("reading");
        unsafe {
            return core::ptr::read_volatile((self.lapic_addr + reg) as *mut u32) as u32;
        }
    }
    fn send_lapic_eoi(lapic_addr: u64) {
        unsafe { core::ptr::write_volatile((lapic_addr + 0xb0) as *mut u32, 0) };
    }
    fn init_lapic(&mut self) {
        let addr = rdmsr(0x1b);

        self.lapic_addr = (addr & 0xfffff000) + HDDM_OFFSET.get_response().unwrap().offset();
        // map lapic

        println!("addr of lapic for cpu: {:#x}", self.lapic_addr);
        let hpet: *mut Hpet = uacpi::table_find_by_signature(HPET_SIGNATURE).unwrap().get_virt_addr() as *mut Hpet;
        
        unsafe {
            core::arch::asm!(
                "mov rax, {0}",
                "mov cr8, rax",
                const 0,
                out("rax") _,
            );

            let q = (*hpet);
            
            // map hpet mmio LOL
            // cur_pagemap.as_mut().unwrap().map(q.base_address as u64 + HDDM_OFFSET.get_response().unwrap().offset() as u64, q.base_address as u64, VMMFlags::KTWRITEALLOWED.bits() | VMMFlags::KTPRESENT.bits()).unwrap();
            let mut it = *((q.address.address as u64
                + HDDM_OFFSET.get_response().unwrap().offset() as u64)
                as *mut u64);

            if it & (1 << 13) != 0 {
                println!("capable");
                it = *((q.address.address as u64
                    + HDDM_OFFSET.get_response().unwrap().offset() as u64
                    + 0x10) as *mut u64);
                *((q.address.address as u64
                    + HDDM_OFFSET.get_response().unwrap().offset() as u64
                    + 0x10) as *mut u64) = *((q.address.address as u64
                    + HDDM_OFFSET.get_response().unwrap().offset() as u64
                    + 0x10) as *mut u64)
                    | 1;
                it = *((q.address.address as u64
                    + HDDM_OFFSET.get_response().unwrap().offset() as u64
                    + 0x0f0) as *mut u64);
                println!("counter is : {it}");

                it = *((q.address.address as u64
                    + HDDM_OFFSET.get_response().unwrap().offset() as u64
                    + 0x0f0) as *mut u64);
                it = *((q.address.address as u64 + HDDM_OFFSET.get_response().unwrap().offset() as u64)
                    as *mut u64);
                it = it >> 32 & 0xFFFFFFFF;
                it = it / 1000000; // TIME IN NANO SECONDS
                self.time_per_tick_hpet = it;
                self.hpet_addr_virt =
                    q.address.address as u64 + HDDM_OFFSET.get_response().unwrap().offset() as u64;
                println!("tim : {it}");
                // 0x100 enables the interrupt, 33 is the interrupt number for a surprious interrupt
                self.write_lapic_register(0xf0, 0x100 | 33);
                // divide by 4
                self.write_lapic_register(0x3e0, 1);
                // one shot timer, unmasked on interrupt 34
                // 1 << 16 masks the timer
                self.write_lapic_register(0x320, 34 | (1 << 16));
                // calibrate the lapic, set the inital count
                self.write_lapic_register(0x380, 0xffffffff);
                self.ksleep(10);
                let mut lapic_ticks_per_10ms = self.read_lapic_register(0x390);
                lapic_ticks_per_10ms = 0xffffffff - lapic_ticks_per_10ms;
                println!("lapic ticks per 10 ms {}", lapic_ticks_per_10ms);
                self.write_lapic_register(0x380, lapic_ticks_per_10ms);
                // unmasked, periodic, interrupt 34
                // (1 << 17) sets to periodic
                // read sdm for more info
                self.write_lapic_register(0x320, 34 | (0 << 16) | (1 << 17));

                core::arch::asm!("sti");
            } else {
                panic!("wtf");
            }
        };
    }
}
