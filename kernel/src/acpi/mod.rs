use core::{alloc::{GlobalAlloc, Layout}, ptr::NonNull, u8};
use ::acpi::{AcpiHandler, AcpiTables, PhysicalMapping};
use limine::request::RsdpRequest;

extern crate alloc;
use alloc::boxed::Box;
use crate::{mem::{phys::{PhysicalAllocator, HDDM_OFFSET}, virt::{cur_pagemap, VMMFlags}, MemoryManager}, println};

#[used]
#[link_section = ".requests"]
static RSDP: RsdpRequest = RsdpRequest::new();
#[derive(Clone, Debug)]
struct acpi;

impl AcpiHandler for acpi
{
    unsafe fn map_physical_region<T>(&self, physical_address: usize, size: usize) -> ::acpi::PhysicalMapping<Self, T> {
        let new = PhysicalMapping::new(physical_address, NonNull::new((physical_address as u64 + HDDM_OFFSET.get_response().unwrap().offset() as u64) as *mut _).unwrap(), size, size, Self.clone());
        return new;
    }
    fn unmap_physical_region<T>(region: &::acpi::PhysicalMapping<Self, T>) {
        
    }
}
pub struct ACPIMANAGER(AcpiTables<acpi>);

impl ACPIMANAGER
{
    pub fn new() -> Self
    {
        ACPIMANAGER(unsafe {AcpiTables::from_rsdp(acpi, (RSDP.get_response().unwrap().address() as u64 - HDDM_OFFSET.get_response().unwrap().offset()) as usize).unwrap()})
    }
}