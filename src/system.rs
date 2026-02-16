pub struct SysStats {
    pub cpu_usage: f32,
    pub mem_used: u64,
    pub mem_total: u64,
    pub processes: Vec<(String, u64)>,
    pub net_in: u64,
    pub net_out: u64,
}
