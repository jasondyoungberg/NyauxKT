use core::alloc::{GlobalAlloc, Layout};
use limine::request::RsdpRequest;
use uacpi::{kernel_api::{set_kernel_api, KernelApi}, Handle, IOAddr, PhysAddr};
extern crate alloc;
use alloc::boxed::Box;
use crate::{mem::{phys::HDDM_OFFSET, MemoryManager}, println};
#[used]
#[link_section = ".requests"]
static RSDP: RsdpRequest = RsdpRequest::new();
struct io_range
{
    base: uacpi::IOAddr,
    len: usize
}
impl io_range
{
    fn new(base: uacpi::IOAddr, len: usize) -> *mut Self
    {
        let mut new = Layout::new::<io_range>();
        unsafe {let x = MemoryManager.alloc(new) as *mut io_range;
            (*x).base = base;
            (*x).len = len;
            return x;
        };
    }
}
pub struct ACPI;

impl KernelApi for ACPI
{
    fn acquire_mutex(&self, mutex: uacpi::Handle, timeout: u16) -> bool {
        true
    }
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        MemoryManager.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        MemoryManager.dealloc(ptr, layout);
    }
    fn create_event(&self) -> uacpi::Handle {
        Handle::new(0)
    }
    fn destroy_event(&self, event: uacpi::Handle) {
        
    }
    fn create_mutex(&self) -> uacpi::Handle {
        Handle::new(0)
    }
    fn destroy_mutex(&self, mutex: uacpi::Handle) {
        
    }
    fn firmware_request(&self, req: uacpi::FirmwareRequest) -> Result<(), uacpi::Status> {
        Ok(())
    }
    fn get_ticks(&self) -> u64 {
        46644
        
    }
    fn install_interrupt_handler(&self, irq: u32, handler: Box<dyn Fn()>)
            -> Result<uacpi::Handle, uacpi::Status> {
        return Err(uacpi::Status::Unimplemented);
    }
    unsafe fn io_map(&self, base: uacpi::IOAddr, len: usize) -> Result<uacpi::Handle, uacpi::Status> {
        let mut yes = io_range::new(base, len);
        return Ok(uacpi::Handle::new(yes as u64))
    }
    unsafe fn io_unmap(&self, handle: Handle) {
        let mut unmap = handle.as_u64() as *mut io_range;
    }
    fn log(&self, log_level: uacpi::LogLevel, string: &str) {
        println!("{}", string);
    }
    unsafe fn map(&self, phys: uacpi::PhysAddr, len: usize) -> *mut core::ffi::c_void {
        (phys.as_u64() + HDDM_OFFSET.get_response().unwrap().offset()) as *mut core::ffi::c_void
    }
    unsafe fn unmap(&self, addr: *mut core::ffi::c_void, len: usize) {
        
    }
    fn release_mutex(&self, mutex: Handle) {
        
    }
    fn uninstall_interrupt_handler(&self, handle: Handle) -> Result<(), uacpi::Status> {
        return Err(uacpi::Status::Unimplemented);
    }
    fn wait_for_event(&self, event: Handle, timeout: u16) -> bool {
        true
    }
    fn stall(&self, usec: u8) {
        
    }
    fn signal_event(&self, event: Handle) {
        
    }
    fn sleep(&self, msec: u8) {
        
    }
    fn schedule_work(&self, work_type: uacpi::WorkType, handler: Box<dyn Fn()>) -> Result<(), uacpi::Status> {
        Err(uacpi::Status::Unimplemented)
    }
    fn reset_event(&self, event: Handle) {
        
    }
    unsafe fn raw_memory_write(
            &self,
            phys: uacpi::PhysAddr,
            byte_width: u8,
            val: u64,
        ) -> Result<(), uacpi::Status> {
        match byte_width
        {
            1 =>
            {
                let w = (phys.as_u64() + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u8;
                unsafe {
                    *w = val as u8;
                }
                return Ok(());
            },
            2 =>
            {
                let w = (phys.as_u64() + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u16;
                unsafe {
                    *w = val as u16;
                }
                return Ok(());
            },
            4 =>
            {
                let w = (phys.as_u64() + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u32;
                unsafe {
                    *w = val as u32;
                }
                return Ok(());
            }
            8 =>
            {
                let w = (phys.as_u64() + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u64;
                unsafe {
                    *w = val;
                }
                return Ok(());
            },
            _ =>
            {
                return Err(uacpi::Status::InvalidArgument);
            }
        }
    }
    unsafe fn raw_memory_read(&self, phys: uacpi::PhysAddr, byte_width: u8) -> Result<u64, uacpi::Status> {
        match byte_width
        {
            1 =>
            {
                let w = (phys.as_u64() + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u8;
                return Ok(*w as u64);
            },
            2 =>
            {
                let w = (phys.as_u64() + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u16;
                return Ok(*w as u64);
            },
            4 =>
            {
                let w = (phys.as_u64() + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u32;
                return Ok(*w as u64);
            }
            8 =>
            {
                let w = (phys.as_u64() + HDDM_OFFSET.get_response().unwrap().offset()) as *mut u64;
                return Ok(*w as u64);
            },
            _ =>
            {
                return Err(uacpi::Status::InvalidArgument);
            }
        }
    }
    unsafe fn pci_read(
            &self,
            address: uacpi::PCIAddress,
            offset: usize,
            byte_width: u8,
        ) -> Result<u64, uacpi::Status> {
        return Err(uacpi::Status::Unimplemented);
    }
    unsafe fn pci_write(
            &self,
            address: uacpi::PCIAddress,
            offset: usize,
            byte_width: u8,
            val: u64,
        ) -> Result<(), uacpi::Status> {
        return Err(uacpi::Status::Unimplemented);;
    }
    unsafe fn io_read(&self, handle: Handle, offset: usize, byte_width: u8) -> Result<u64, uacpi::Status> {
        let mut rng = handle.as_u64() as *mut io_range;
        if offset >= (*rng).len
        {
            return Err(uacpi::Status::InvalidArgument);
        }
        return self.raw_io_read((*rng).base, byte_width);
    }
    unsafe fn io_write(
            &self,
            handle: Handle,
            offset: usize,
            byte_width: u8,
            val: u64,
        ) -> Result<(), uacpi::Status> {
            let mut rng = handle.as_u64() as *mut io_range;
            if offset >= (*rng).len
            {
                return Err(uacpi::Status::InvalidArgument);
            }
            return self.raw_io_write((*rng).base, byte_width, val);
    }
    unsafe fn raw_io_read(&self, addr: IOAddr, byte_width: u8) -> Result<u64, uacpi::Status> {
        if !byte_width.is_power_of_two()
        {
            return Err(uacpi::Status::InvalidArgument);
        }
        if byte_width > 8
        {
            return Err(uacpi::Status::InvalidArgument);
        }
        match byte_width
        {
            1 =>
            {
                let mut x: u8 = 0;
                core::arch::asm!("inb %dx, %al", in("dx") addr.as_u64(), out("al") x, options(att_syntax));
                return Ok(x as u64);

            },
            2 => {
                let mut x = 0;
                core::arch::asm!("inw %dx, %ax", in("dx") addr.as_u64(), out("ax") x, options(att_syntax));
                return Ok(x);
            }
            4 => {
                let mut x = 0;
                core::arch::asm!("inl %dx, %eax", in("dx") addr.as_u64(), out("eax") x, options(att_syntax));
                return Ok(x);
            }
            8 => {
                return Err(uacpi::Status::InvalidArgument);
            }
            _ =>
            {
                return Err(uacpi::Status::InvalidArgument);
            }
        }

    }
    unsafe fn raw_io_write(&self, addr: IOAddr, byte_width: u8, val: u64) -> Result<(), uacpi::Status> {
        if !byte_width.is_power_of_two()
        {
            return Err(uacpi::Status::InvalidArgument);
        }
        if byte_width > 8
        {
            return Err(uacpi::Status::InvalidArgument);
        }
        match byte_width
        {
            1 =>
            {

                core::arch::asm!("outb %al, %dx", in("al") val as u8, in("dx") addr.as_u64(), options(att_syntax));
                return Ok(());

            },
            2 => {
                
                core::arch::asm!("outw %ax, %dx", in("ax") val, in("dx") addr.as_u64(), options(att_syntax));
                return Ok(());
            }
            4 => {
                
                core::arch::asm!("outl %eax, %dx", in("eax") val, in("dx") addr.as_u64(), options(att_syntax));
                return Ok(());
            }
            8 => {
                return Err(uacpi::Status::InvalidArgument);
            }
            _ =>
            {
                return Err(uacpi::Status::InvalidArgument);
            }
        }
    }
}
impl ACPI
{
    pub fn init()
    {
        let mut x = ACPI {};
        set_kernel_api(alloc::sync::Arc::new(x));
        let res = uacpi::init(PhysAddr::new(RSDP.get_response().unwrap().address() as u64 - HDDM_OFFSET.get_response().unwrap().offset()), uacpi::LogLevel::TRACE, false);
        if res.is_err()
        {
           panic!("failed")
        }
        println!("Success!");

    }
}