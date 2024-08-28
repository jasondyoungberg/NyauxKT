#![no_std]
#![no_main]

use core::{arch::asm};

use limine::request::FramebufferRequest;
use limine::BaseRevision;
use flanterm_bindings::{self, flanterm_fb_init, flanterm_write};
use NyauxKT::{mem::phys::{HDDM_OFFSET, PMM}, println, utils::{self, KTError}, TERM};
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
            let x = PMM.alloc();
            match x {
                Ok(mut e) => {
                    e = (e as u64 + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u8;
                    *e = 3;
                    println!("e is {}", 3);
                    
                    PMM.dealloc(e);
                    
                    println!("Complete!");
                },
                Err(e) =>
                {

                }
            }
            
        }
    }

    hcf();
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
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
