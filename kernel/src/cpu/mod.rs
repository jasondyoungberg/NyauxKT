pub mod lapic;
extern crate alloc;
use crate::mem::phys::HDDM_OFFSET;
use crate::utils::rdmsr;
use crate::println;
use alloc::vec::Vec;
use lapic::LAPIC;
use limine::request::SmpRequest;
#[link_section = ".requests"]
static SMP: SmpRequest = SmpRequest::new();
#[derive(Debug)]
pub struct CPU {
    pub lapic_addr: u64,
    pub lapic_id: u32
    
}

impl CPU {
    fn construct_cpu() -> Self {
        let addr = rdmsr(0x1b);

        let shit = (addr & 0xfffff000) + HDDM_OFFSET.get_response().unwrap().offset();
        let id = CPU::read_lapic_id(shit);
        Self {
            lapic_id: id,
            lapic_addr: shit
        }
    }
}
static mut CPUS: Option<Vec<CPU>> = None;

pub fn init_smp() {
    
    unsafe {
        CPUS = Some(Vec::new());
    }
    println!("bsp lapic id {}", SMP.get_response().unwrap().bsp_lapic_id());
    for i in SMP.get_response().unwrap().cpus() {
        
        println!("Found CPU with id {}", i.lapic_id);
        unsafe {
            let w = CPU::construct_cpu();
            println!("Created CPU Structure {:?}", w);
            CPUS.as_mut().unwrap().push(w);
        }
    }

}