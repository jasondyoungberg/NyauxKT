#![no_std]
#![no_main]
#![feature(naked_functions)]


use core::{arch::asm};

use limine::request::FramebufferRequest;
use limine::BaseRevision;
use flanterm_bindings::{self, flanterm_fb_init, flanterm_write};
use NyauxKT::{cpu::{self, lapic::{self, LAPIC}}, idt::InterruptManager, mem::phys::{HDDM_OFFSET, PMM}, println, utils::{self, KTError}, TERM};
use core::fmt::Write;
use owo_colors::OwoColorize;
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
unsafe extern "C" fn kmain() -> ! {
    // All limine requests must also be referenced in a called function, otherwise they may be
    // removed by the linker.
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
            let mut cpu = cpu::CPU {cpu_id: 0,lapic_addr: 0};
            cpu.init_lapic();
        }
    }

    hcf();
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    
    TERM.lock().clear_screen(0xFF0000);
    println!("KT Kernel Panic!: {}", _info);
    hcf();
}

fn hcf() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}
