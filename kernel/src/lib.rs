#![cfg_attr(not(test), no_std)]
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
#![feature(const_nonnull_new)]
#![feature(const_option)]
#![feature(naked_functions)]
use lazy_static::lazy_static;
use spin::Mutex;
use utils::NyauxTerm;
pub mod acpi;
pub mod cpu;
pub mod drivers;
pub mod fs;
pub mod idt;
pub mod mem;
pub mod sched;
pub mod utils;

lazy_static! {
    pub static ref TERM: Mutex<NyauxTerm> = NyauxTerm::new_none();
}
