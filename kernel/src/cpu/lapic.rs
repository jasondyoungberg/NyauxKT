use acpi::{hpet::HpetTable, AcpiTable, HpetInfo};

use crate::{acpi::ACPIMANAGER, mem::phys::HDDM_OFFSET, println, utils::rdmsr};

use super::CPU;

pub trait LAPIC
{
    fn init_lapic(&mut self, e: &mut ACPIMANAGER);
}
impl LAPIC for CPU
{
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
            let it = (q.base_address + HDDM_OFFSET.get_response().unwrap().offset() as usize);
            if it & (1 << 13) != 0
            {
                println!("capable");
            }
            else {
                panic!("wtf");
            }
        };
    }
}