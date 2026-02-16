use sysinfo::{CpuExt, System, SystemExt};

pub struct SysStats {
    pub cpu_usage: f32,
    pub mem_used: u64,
    pub mem_total: u64,
}

pub fn get_stats(sys: &mut System) -> SysStats {
    sys.refresh_all();

    // Calculate average CPU usage across all cores
    let cpu: f32 =
        sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;

    SysStats {
        cpu_usage: cpu,
        mem_used: sys.used_memory(),
        mem_total: sys.total_memory(),
    }
}
