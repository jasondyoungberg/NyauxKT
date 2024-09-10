
use core::{
    alloc::{GlobalAlloc, Layout}, ffi::c_void, ptr::NonNull, u8
};
use limine::request::RsdpRequest;
use uacpi::{kernel_api::{self, set_kernel_api, KernelApi}, CpuFlags, IOAddr, PhysAddr};

extern crate alloc;
use crate::{
    mem::{
        phys::{PhysicalAllocator, HDDM_OFFSET},
        virt::{cur_pagemap, VMMFlags},
        MemoryManager,
    }, print, println, utils::{self, read_from_portu16, read_from_portu32, read_from_portu8, write_to_portu16, write_to_portu32, write_to_portu8}
};
use alloc::boxed::Box;
struct io_range
{
    base: IOAddr,
    len: usize
}
impl io_range
{
    fn new(base: IOAddr, len: usize) -> Box<io_range>
    {
        Box::new(
            io_range
            {
                base: base,
                len: len
            }
        )
    }
}
#[used]
#[link_section = ".requests"]
static RSDP: RsdpRequest = RsdpRequest::new();
#[derive(Clone, Debug)]
pub struct acpi;
impl uacpi::kernel_api::KernelApi for acpi
{
    fn acquire_mutex(&self, mutex: uacpi::Handle, timeout: u16) -> bool {
        true
    }
    fn acquire_spinlock(&self, lock: uacpi::Handle) -> uacpi::CpuFlags {
        CpuFlags::new(0)
    }
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        alloc::alloc::alloc(layout)
    }
    fn create_event(&self) -> uacpi::Handle {
        uacpi::Handle::new(1)
    }
    fn create_mutex(&self) -> uacpi::Handle {
        uacpi::Handle::new(1)
    }
    fn create_spinlock(&self) -> uacpi::Handle {
        uacpi::Handle::new(1)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        alloc::alloc::dealloc(ptr, layout);
    }
    fn destroy_event(&self, event: uacpi::Handle) {
        
    }
    fn destroy_mutex(&self, mutex: uacpi::Handle) {
        
    }
    fn destroy_spinlock(&self, lock: uacpi::Handle) {
        
    }
    fn firmware_request(&self, req: uacpi::FirmwareRequest) -> Result<(), uacpi::Status> {
        Err(uacpi::Status::Unimplemented)
    }
    fn get_thread_id(&self) -> uacpi::ThreadId {
        uacpi::ThreadId::new(0 as *mut c_void)
    }
    fn get_ticks(&self) -> u64 {
        46644
    }
    fn install_interrupt_handler(&self, irq: u32, handler: Box<dyn Fn()>,
        ) -> Result<uacpi::Handle, uacpi::Status> {
            println!("h");
        Ok(uacpi::Handle::new(1))
    }
    unsafe fn io_map(&self, base: uacpi::IOAddr, len: usize) -> Result<uacpi::Handle, uacpi::Status> {
        
        Ok(
            uacpi::Handle::new(base.as_u64())
            
        )
        
    }
    unsafe fn io_read(&self, handle: uacpi::Handle, offset: usize, byte_width: u8) -> Result<u64, uacpi::Status> {
        let it = handle.as_u64();
        self.raw_io_read(IOAddr::new(it + offset as u64), byte_width)
    }
    unsafe fn io_unmap(&self, handle: uacpi::Handle) {
        println!("a");
        todo!()
    }
    unsafe fn io_write(
            &self,
            handle: uacpi::Handle,
            offset: usize,
            byte_width: u8,
            val: u64,
        ) -> Result<(), uacpi::Status> {
            let it = handle.as_u64();
            self.raw_io_write(IOAddr::new(it + offset as u64), byte_width, val)
            
    }
    fn log(&self, log_level: uacpi::LogLevel, string: &str) {
        print!("{}", string);
    }
    unsafe fn map(&self, phys: uacpi::PhysAddr, len: usize) -> *mut core::ffi::c_void {
        (phys.as_u64() + HDDM_OFFSET.get_response().unwrap().offset()) as *mut _
    }
    unsafe fn pci_read(
            &self,
            address: uacpi::PCIAddress,
            offset: usize,
            byte_width: u8,
        ) -> Result<u64, uacpi::Status> {
        Err(uacpi::Status::Unimplemented)
    }
    unsafe fn pci_write(
            &self,
            address: uacpi::PCIAddress,
            offset: usize,
            byte_width: u8,
            val: u64,
        ) -> Result<(), uacpi::Status> {
        Err(uacpi::Status::Unimplemented)
    }
    unsafe fn raw_io_read(&self, addr: uacpi::IOAddr, byte_width: u8) -> Result<u64, uacpi::Status> {
        let e = addr.as_u64();
        match byte_width
        {
            1 =>
            {
                let val = read_from_portu8(e as u16);
                Ok(val as u64)
            }
            2 =>
            {
                let val = read_from_portu16(e as u16);
                Ok(val as u64)
            }
            4 =>
            {
                let val = read_from_portu32(e as u16);
                Ok(val as u64)
            }
            8 =>
            {
                Err(uacpi::Status::InvalidArgument)
            },
            _ => Err(uacpi::Status::InvalidArgument)
        }
    }
    unsafe fn raw_io_write(&self, addr: uacpi::IOAddr, byte_width: u8, val: u64) -> Result<(), uacpi::Status> {
        let e = addr.as_u64();
        match byte_width
        {
            1 =>
            {
                write_to_portu8(e as u16, val as u8);
                Ok(())
            }
            2 =>
            {
                write_to_portu16(e as u16, val as u16);
                Ok(())
            }
            4 =>
            {
                write_to_portu32(e as u16, val as u32);
                Ok(())
            }
            8 =>
            {
                Err(uacpi::Status::InvalidArgument)
            },
            _ => Err(uacpi::Status::InvalidArgument)
        }
    }
    unsafe fn raw_memory_read(&self, phys: uacpi::PhysAddr, byte_width: u8) -> Result<u64, uacpi::Status> {
        let virt = phys.as_u64() + HDDM_OFFSET.get_response().unwrap().offset();
        match byte_width
        {
            1 => Ok((*(virt as *mut u8)) as u64),
            2 => Ok((*(virt as *mut u16)) as u64),
            4 => Ok((*(virt as *mut u32)) as u64),
            8 => Ok((*(virt as *mut u64)) as u64),
            _ => Err(uacpi::Status::InvalidArgument)
        }
    }
    unsafe fn raw_memory_write(
            &self,
            phys: uacpi::PhysAddr,
            byte_width: u8,
            val: u64,
        ) -> Result<(), uacpi::Status> {
            let virt = phys.as_u64() + HDDM_OFFSET.get_response().unwrap().offset();
        match byte_width
        {
            1 => {
                *(virt as *mut u8) = val as u8;
                Ok(())
            },
            2 => {
                *(virt as *mut u16) = val as u16;
                Ok(())
            },
            4 => {
                *(virt as *mut u32) = val as u32;
                Ok(())
            },
            8 => {
                *(virt as *mut u64) = val as u64;
                Ok(())
            },
            _ => Err(uacpi::Status::InvalidArgument)
        }
    }
    fn release_mutex(&self, mutex: uacpi::Handle) {
        
    }
    fn release_spinlock(&self, lock: uacpi::Handle, cpu_flags: uacpi::CpuFlags) {
        
    }
    fn reset_event(&self, event: uacpi::Handle) {
        
    }
    fn schedule_work(&self, work_type: uacpi::WorkType, handler: Box<dyn Fn()>) -> Result<(), uacpi::Status> {
        Err(uacpi::Status::Unimplemented)
    }
    fn signal_event(&self, event: uacpi::Handle) {
        
    }
    fn sleep(&self, msec: u8) {
        
    }
    fn stall(&self, usec: u8) {
        
    }
    fn uninstall_interrupt_handler(&self, handle: uacpi::Handle) -> Result<(), uacpi::Status> {
        Err(uacpi::Status::Unimplemented)
    }
    unsafe fn unmap(&self, addr: *mut core::ffi::c_void, len: usize) {
        
    }
    fn wait_for_event(&self, event: uacpi::Handle, timeout: u16) -> bool {
        false
    }
    fn wait_for_work_completion(&self) -> Result<(), uacpi::Status> {
        Err(uacpi::Status::Unimplemented)
    }
}
pub struct ACPIMANAGER(alloc::sync::Arc<dyn KernelApi>);
impl ACPIMANAGER
{
    pub fn new() -> ACPIMANAGER
    {
        let x = ACPIMANAGER(alloc::sync::Arc::new(acpi));
        set_kernel_api(alloc::sync::Arc::clone(&x.0));
        uacpi::init(uacpi::PhysAddr::new(RSDP.get_response().unwrap()
        .address() as u64 - HDDM_OFFSET.get_response().unwrap().offset() as u64), uacpi::LogLevel::TRACE, false);
        
        uacpi::namespace_load().unwrap();
        uacpi::namespace_initialize().unwrap();
        return x;
    }
}