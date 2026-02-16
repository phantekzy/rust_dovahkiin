use sysinfo::{CpuExt, System, SystemExt};

#[derive(Debug)]
pub struct SysStats {
    pub cpu_usage: f32,
    pub mem_used: u64,
    pub mem_total: u64,
}
