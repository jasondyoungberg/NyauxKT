
use core::{arch::global_asm, ffi::c_void};

use crate::println;


#[repr(C)]
#[derive(Clone, Copy)]
struct GateDescriptor {
    offset: u16,
    seg: u16,
    ist_and_reversed: u8,
    flags: u8,
    offset_mid: u16,
    offset_hi: u32,
    reversed: u32,
}
impl GateDescriptor
{
    const fn new() -> Self {
        return Self 
        {
            offset: 0,
            seg: 0,
            ist_and_reversed: 0,
            flags: 0,
            offset_mid: 0,
            offset_hi: 0,
            reversed: 0
        }
    }
}
static mut IDT: [GateDescriptor; 256] = [GateDescriptor::new(); 256];
#[repr(C, packed)]
#[derive(Debug)]
struct IDTR {
    size: u16,
    offset: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct Registers {
    // Pushed by wrapper
    int: usize,

    // Pushed by push_gprs in crate::arch::x86_64
    r15: usize,
    r14: usize,
    r13: usize,
    r12: usize,
    r11: usize,
    r10: usize,
    r9: usize,
    r8: usize,
    rbp: usize,
    rdi: usize,
    rsi: usize,
    rdx: usize,
    rcx: usize,
    rbx: usize,
    rax: usize,

    // Pushed by interrupt
    rip: usize,
    cs: usize,
    rflags: usize,
    rsp: usize,
    ss: usize,
}
extern "C" fn exception_handler(registers: u64)
{
    let got_registers = registers as *mut Registers;
    unsafe {
        panic!("crash with register rip at {:#x}", (*got_registers).rip);
    }
}
pub fn idt_set_gate(num: u8, function_ptr: usize) {
    let base = function_ptr;
    unsafe {
        IDT[num as usize] = GateDescriptor {
            offset: (base & 0xFFFF) as u16,
            offset_mid: ((base >> 16) & 0xFFFF) as u16,
            offset_hi: ((base >> 32) & 0xFFFFFFFF) as u32,
            seg: 0x28,
            ist_and_reversed: 0,
            reversed: 0,
            flags: 0xEE,
        };
    }

    
}

macro_rules! exception_function {
    ($code:expr, $handler:ident) => {
        
        #[naked]
        extern "C" fn $handler() {

            unsafe {
                core::arch::asm!(
                    "push rax",
                    "push rbx",
                    "push rcx",
                    "push rdx",
                    "push rsi",
                    "push rdi",
                    "push rbp",
                    "push r8",
                    "push r9",
                    "push r10",
                    "push r11",
                    "push r12",
                    "push r13",
                    "push r14",
                    "push r15",
                    "push {0}",
                    "mov rdi, rsp",
                    "call {1}",
                    "add rsp, 8",
                    "mov rsp, rdi",
                    "pop r15",
                    "pop r14",
                    "pop r13",
                    "pop r12",
                    "pop r11",
                    "pop r10",
                    "pop r9",
                    "pop r8",
                    "pop rbp",
                    "pop rdi",
                    "pop rsi",
                    "pop rdx",
                    "pop rcx",
                    "pop rbx",
                    "pop rax",
                    const $code,
                    sym exception_handler,
                    options(noreturn)
                );
            };



            
        }
    };
}

exception_function!(0x00, div_error);
exception_function!(0x06, invalid_opcode);
exception_function!(0x08, double_fault);
exception_function!(0x0D, general_protection_fault);
exception_function!(0x0E, page_fault);
exception_function!(0xFF, generic_handler);

static mut IDTR: IDTR = IDTR { offset: 0, size: 0 };
extern "C" {
    fn init_idt(a: *const c_void);
    
}
pub struct InterruptManager
{

}
impl InterruptManager
{
    pub fn start_idt()
    {
        idt_set_gate(0x00, div_error as usize);
        idt_set_gate(0x06, invalid_opcode as usize);
        idt_set_gate(0x08, double_fault as usize);
        idt_set_gate(0x0D, general_protection_fault as usize);
        idt_set_gate(0x0E, page_fault as usize);
        unsafe {IDTR.offset = IDT.as_ptr() as u64};
        
        unsafe {IDTR.size = ((core::mem::size_of::<GateDescriptor>() * 256) - 1)  as u16};
        
        
        unsafe {
            core::arch::asm!(
                "lidt [{}]",
                in(reg) core::ptr::addr_of!(IDTR)
            );
        }
        

    }
}