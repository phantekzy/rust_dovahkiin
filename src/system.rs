use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System};

pub struct SysStats {
    pub cpu_usage: f32,
    pub mem_used: u64,
    pub mem_total: u64,
    pub processes: Vec<(String, u64)>,
    pub net_in: u64,
    pub net_out: u64,
}

pub fn get_stats(sys: &mut System) -> SysStats {
    sys.refresh_all();

    //  Average CPU
    let cpu = sys.cpus().iter().map(|c| c.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;

    // Top 5 processes by memory
    let mut procs: Vec<_> = sys.processes().values().collect();
    procs.sort_by(|a, b| b.memory().cmp(&a.memory()));
    let top_procs = procs
        .iter()
        .take(5)
        .map(|p| (p.name().to_string(), p.memory() / 1024 / 1024))
        .collect();

    //  Network
    let mut total_in = 0;
    let mut total_out = 0;
    for (_, data) in sys.networks() {
        total_in += data.received();
        total_out += data.transmitted();
    }

    SysStats {
        cpu_usage: cpu,
        mem_used: sys.used_memory(),
        mem_total: sys.total_memory(),
        processes: top_procs,
        net_in: total_in,
        net_out: total_out,
    }
}
