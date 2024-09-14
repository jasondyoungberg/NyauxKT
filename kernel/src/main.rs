#![feature(naked_functions)]
#![allow(
    unused,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    unused_variables,
    unused_mut,
    unused_parens,
    unused_must_use,
    unused_results,
    non_camel_case_types,
    
)]
#![cfg_attr(not(test), no_std, no_main)]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{alloc as other, vec};
use core::fmt::Write;
use flanterm_bindings::{self, flanterm_fb_init, flanterm_write};
use limine::request::FramebufferRequest;
use limine::BaseRevision;
use owo_colors::OwoColorize;

use NyauxKT::mem::{global, MemoryManager};
use NyauxKT::{
    acpi::ACPIMANAGER,
    cpu::{
        self,
        lapic::{self, LAPIC},
    },
    idt::InterruptManager,
    mem::phys::{HDDM_OFFSET, PMM},
    println,
    utils::{self, KTError},
    TERM,
};
/// Sets the base revision to the latest revision supported by the crate.
/// See specification for further info.
// Be sure to mark all limine requests with #[used], otherwise they may be removed by the compiler.
#[used]
// The .requests section allows limine to find the requests faster and more safely.
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[no_mangle]
#[cfg(not(test))]
unsafe extern "C" fn kmain() -> ! {
    // All limine requests must also be referenced in a called function, otherwise they may be
    // removed by the linker.

    use NyauxKT::{drivers::apic::apic_init, fs::USTAR::ustarinit};
    assert!(BASE_REVISION.is_supported());

    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            TERM.lock().init_basic(framebuffer);
            NyauxKT::mem::gdt::init_gdt();
            NyauxKT::mem::phys::PhysicalAllocator::new();
            
            println!("PMM [{}]", "Okay".bright_green());
            println!("GDT [{}]", "Okay".bright_green());
            NyauxKT::mem::virt::PageMap::new_inital();
            println!("VMM [{}]", "Okay".bright_green());
            InterruptManager::start_idt();
            println!("IDT [{}]", "Okay".bright_green());
            let mut cpu = cpu::CPU {
                cpu_id: 0,
                lapic_addr: 0,
                hpet_addr_virt: 0,
                time_per_tick_hpet: 0,
            };

            let mut x = ACPIMANAGER::new();

            cpu.init_lapic();
            println!("LAPIC [{}]", "Okay".bright_green());
            ustarinit();
            apic_init();
            println!("USTAR [{}]", "Okay".bright_green());
            println!("APIC [{}]", "Okay".bright_green());
            
            
        }
    }

    hcf();
}

#[cfg(not(test))]
#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    //TERM.lock().clear_screen(0xFF0000);
    println!("KT Kernel Panic!: {}", _info);
    hcf();
}
fn hcf() -> ! {
    unsafe {
        loop {
            core::arch::asm!("hlt");
        }
    }
}
