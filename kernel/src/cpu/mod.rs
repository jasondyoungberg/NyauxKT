pub mod lapic;

pub struct CPU
{
    pub cpu_id: u64,
    pub lapic_addr: u64,
    pub time_per_tick_hpet: u64,
    pub hpet_addr_virt: u64
}