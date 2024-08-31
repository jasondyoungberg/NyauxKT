use crate::{println, utils::rdmsr};

use super::CPU;

pub trait LAPIC
{
    fn init_lapic(&mut self);
}
impl LAPIC for CPU
{
    fn init_lapic(&mut self) {
        let addr = rdmsr(0x1b);
        self.lapic_addr = addr & 0xfffff000;
        println!("addr of lapic for cpu: {:#x}", self.lapic_addr);
    }
}