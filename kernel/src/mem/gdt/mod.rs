use core::{arch::global_asm, ffi::c_void, ptr::addr_of};
global_asm!(include_str!("flush.s"));
#[repr(C, packed)]

struct GDTR
{
    size: u16,
    offset: u64
}
impl GDTR
{
    fn new(table: u64, size: u16) -> GDTR
    {
        GDTR { size: size - 1, offset: table }
    }
}
#[repr(C, packed)]
struct GDTDescriptor
{
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access_byte: u8,
    flag_an_hilim: u8,
    base_hi: u8 
}
// this is useless lol
static mut gdt: [u64; 9] = [0,0,0,0,0,0,0,0,0];
static mut gdtptr: GDTR = GDTR{size: 0, offset: 0};
extern "C"
{
    fn gdt_flush(a: *const c_void);
}
pub fn init_gdt()
{
    unsafe {
        gdt[0] = 0x0;
        gdt[1] = 0x00009a000000ffff;
        gdt[2] = 0x000093000000ffff;
        gdt[3] = 0x00cf9a000000ffff;
        gdt[4] = 0x00cf93000000ffff;
        gdt[5] = 0x00af9b000000ffff;
        gdt[6] = 0x00af93000000ffff;
        gdt[7] = 0x00aff3000000ffff;
        gdt[8] = 0x00affb000000ffff;
        // wtf is this rust lmao

        gdtptr.size = size_of_val(&*core::ptr::addr_of!(gdt)) as u16;
        gdtptr.offset = addr_of!(gdt) as u64;
        gdt_flush(addr_of!(gdtptr) as *const _);
    }
}