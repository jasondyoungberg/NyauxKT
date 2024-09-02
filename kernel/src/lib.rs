#![no_std]
#![feature(const_nonnull_new)]
#![feature(const_option)]
#![feature(naked_functions)]
use lazy_static::lazy_static;
use spin::Mutex;
use utils::NyauxTerm;
pub mod utils;
pub mod mem;
pub mod idt;
pub mod cpu;
pub mod acpi;

lazy_static!{
    pub static ref TERM: Mutex<NyauxTerm> = NyauxTerm::new_none();
}