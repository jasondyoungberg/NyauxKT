use acpi::{hpet::HpetTable, AcpiTable, HpetInfo};

use crate::{acpi::ACPIMANAGER, mem::{phys::{align_down, align_up, HDDM_OFFSET}, virt::{cur_pagemap, VMMFlags}}, println, utils::rdmsr};

use super::CPU;

pub trait LAPIC
{
    unsafe fn ksleep(&self, ms: u64);
    fn init_lapic(&mut self, e: &mut ACPIMANAGER);
}
impl LAPIC for CPU
{
    unsafe fn ksleep(&self, ms: u64)
    {
        let pol_start = *((self.hpet_addr_virt + 0x0f0) as *mut u64);
        let pol_cur = (self.hpet_addr_virt + 0x0f0) as *mut u64;
        while ((*(pol_cur) - pol_start) * self.time_per_tick_hpet < ms * 1000000)
        {
           
                core::arch::asm!("nop");
            
        }
    }
    fn init_lapic(&mut self, e: &mut ACPIMANAGER) {
        let addr = rdmsr(0x1b);
        self.lapic_addr = addr & 0xfffff000;
        println!("addr of lapic for cpu: {:#x}", self.lapic_addr);
        let table = e.0.find_table::<HpetTable>().unwrap_or_else(|_| panic!("fuck"));
        unsafe {
            core::arch::asm!(
                "mov rax, {0}",
                "mov cr8, rax",
                const 0,
                out("rax") _,
            );
            
            
            let q =  HpetInfo::new(&e.0).unwrap();
            println!("base addr: {:#x}",q.base_address);
            // map hpet mmio LOL
            cur_pagemap.as_mut().unwrap().map(q.base_address as u64 + HDDM_OFFSET.get_response().unwrap().offset() as u64, q.base_address as u64, VMMFlags::KTWRITEALLOWED.bits() | VMMFlags::KTPRESENT.bits()).unwrap();
            let mut it = *((q.base_address as u64 + HDDM_OFFSET.get_response().unwrap().offset() as u64) as *mut u64);
            
            if it & (1 << 13) != 0
            {
                println!("capable");
                it = *((q.base_address as u64 + HDDM_OFFSET.get_response().unwrap().offset() as u64 + 0x10) as *mut u64);
                *((q.base_address as u64 + HDDM_OFFSET.get_response().unwrap().offset() as u64 + 0x10) as *mut u64) = *((q.base_address as u64 + HDDM_OFFSET.get_response().unwrap().offset() as u64 + 0x10) as *mut u64) | 1;
                it = *((q.base_address as u64 + HDDM_OFFSET.get_response().unwrap().offset() as u64 + 0x0f0) as *mut u64);
                println!("counter is : {it}");
                
                it = *((q.base_address as u64 + HDDM_OFFSET.get_response().unwrap().offset() as u64 + 0x0f0) as *mut u64);
                it = *((q.base_address as u64 + HDDM_OFFSET.get_response().unwrap().offset() as u64) as *mut u64);
                it = it >> 32 & 0xFFFFFFFF;
                it = it / 1000000; // TIME IN NANO SECONDS
                self.time_per_tick_hpet = it;
                self.hpet_addr_virt = q.base_address as u64 + HDDM_OFFSET.get_response().unwrap().offset() as u64;
                println!("tim : {it}");
                self.ksleep(1000);
                println!("woah");

            }
            else {
                panic!("wtf");
            }
        };
    }
}