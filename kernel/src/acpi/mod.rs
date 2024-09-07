use core::{alloc::{GlobalAlloc, Layout}, ptr::NonNull, u8};
use limine::request::RsdpRequest;

extern crate alloc;
use alloc::boxed::Box;
use crate::{mem::{phys::HDDM_OFFSET, virt::{cur_pagemap, VMMFlags}, MemoryManager}, println};

#[used]
#[link_section = ".requests"]
static RSDP: RsdpRequest = RsdpRequest::new();
