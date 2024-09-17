pub mod lapic;
extern crate alloc;
use crate::mem::gdt::init_gdt;
use crate::mem::phys::HDDM_OFFSET;
use crate::println;
use crate::utils::rdmsr;
use alloc::vec::Vec;
use lapic::LAPIC;
use limine::request::SmpRequest;
#[link_section = ".requests"]
static SMP: SmpRequest = SmpRequest::new();
#[derive(Debug)]
pub struct CPU {
    pub lapic_addr: u64,
    pub lapic_id: u32,
}

impl CPU {
    fn construct_cpu(id: u32) -> Self {
        let mut ee = 0;
        if id == 0 {
            let addr = rdmsr(0x1b);

            let shit = (addr & 0xfffff000) + HDDM_OFFSET.get_response().unwrap().offset();
            ee = shit;
        }
        Self {
            lapic_id: id,
            lapic_addr: ee,
        }
    }
}
static mut CPUS: Option<Vec<CPU>> = None;

unsafe extern "C" fn init_cpu(e: &limine::smp::Cpu) -> ! {
    core::arch::asm!("cli");
    
    let mut q: Option<&mut CPU> = None;
    for i in CPUS.as_mut().unwrap().iter_mut() {
        if i.lapic_id == e.lapic_id {
            let addr = rdmsr(0x1b);

            let shit = (addr & 0xfffff000) + HDDM_OFFSET.get_response().unwrap().offset();
            i.lapic_addr = shit;
            q = Some(i);
        }
    }
    init_gdt();
    crate::idt::InterruptManager::start_idt();
    
    crate::sched::ITIS.as_mut().unwrap().create_queue(e.lapic_id);
    q.unwrap().init_lapic();
    core::arch::asm!("sti");
    unsafe {
        loop {
            core::arch::asm!("hlt");
        }
    }
}
pub fn init_smp() {
    unsafe {
        CPUS = Some(Vec::new());
    }
    println!(
        "bsp lapic id {}",
        SMP.get_response().unwrap().bsp_lapic_id()
    );
    for i in SMP.get_response().unwrap().cpus() {
        println!("Found CPU with id {}", i.lapic_id);
        unsafe {
            let mut w = CPU::construct_cpu(i.lapic_id);
            if w.lapic_id == 0 {
                w.init_lapic();
            }
            
            CPUS.as_mut().unwrap().push(w);
        }
    }
    for i in SMP.get_response().unwrap().cpus() {
        i.goto_address.write(init_cpu)
    }
}
