#![feature(naked_functions)]
#![allow(
    non_camel_case_types,
    dead_code,
    non_snake_case,
    unused_imports,
    non_upper_case_globals,
    unused_unsafe,
    unreachable_code,
    unused_attributes,
    unused_variables,
    unused_parens,
    unused_mut,
    unused_assignments
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

    use NyauxKT::{cpu::init_smp, drivers::apic::apic_init, fs::{self, USTAR::ustarinit}, sched, serial_println};
    assert!(BASE_REVISION.is_supported());

    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            TERM.lock().init_basic(&framebuffer);
            NyauxKT::mem::gdt::init_gdt();
            NyauxKT::mem::phys::PhysicalAllocator::new();

            NyauxKT::mem::virt::PageMap::new_inital();
            
            TERM.lock().deinit();
            TERM.lock().init(&framebuffer);
            println!("PMM [{}]", "Okay".bright_green());
            println!("GDT [{}]", "Okay".bright_green());
            println!("VMM [{}]", "Okay".bright_green());
            InterruptManager::start_idt();
            println!("IDT [{}]", "Okay".bright_green());

            println!("Welcome to Nyaux!.");

            let mut x = ACPIMANAGER::new();

            ustarinit();
            apic_init();
            println!("USTAR [{}]", "Okay".bright_green());
            println!("APIC [{}]", "Okay".bright_green());

            serial_println!("Everything is {}", "Okay".bright_green());
            core::arch::asm!("cli");
            sched::sched_init();
            fs::devfs::devfs_init();
            println!("dev fs created");
            serial_println!("dev fs created!");
            
            init_smp();
            core::arch::asm!("sti");
            
        }
    }

    hcf();
}

#[cfg(not(test))]
#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    //TERM.lock().clear_screen(0xFF0000);

    use NyauxKT::serial_println;
    println!("KT Kernel Panic!: {}", _info);
    serial_println!("NT Kernel Panic!: {}", _info);
    hcf();
}
pub fn hcf() -> ! {
    unsafe {
        loop {
            core::arch::asm!("hlt");
        }
    }
}
