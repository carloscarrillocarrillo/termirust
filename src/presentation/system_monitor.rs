use sysinfo::System;
use crate::infrastructure::logging::log_system_stats;

pub struct SystemMonitor;

impl SystemMonitor {
    pub fn new() -> Self {
        Self
    }

    pub fn get_system_stats(&self) -> (f64, f64, (u64, u64)) {
        let mut sys = System::new_all();
        sys.refresh_all();
        
        let memory_usage = self.calculate_memory_usage(&sys);
        let cpu_usage = self.calculate_cpu_usage(&sys);
        let network_usage = self.simulate_network_usage();
        
        self.log_stats_periodically(memory_usage, cpu_usage, network_usage);
        
        (memory_usage, cpu_usage, network_usage)
    }

    fn calculate_memory_usage(&self, sys: &System) -> f64 {
        let total_memory = sys.total_memory() as f64;
        let used_memory = sys.used_memory() as f64;
        
        if total_memory > 0.0 {
            (used_memory / total_memory) * 100.0
        } else {
            0.0
        }
    }

    fn calculate_cpu_usage(&self, sys: &System) -> f64 {
        sys.global_cpu_info().cpu_usage() as f64
    }

    fn simulate_network_usage(&self) -> (u64, u64) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let received = (timestamp % 1000) * 1024 * 1024;
        let transmitted = (timestamp % 500) * 1024 * 1024;
        
        (received, transmitted)
    }

    fn log_stats_periodically(&self, memory_usage: f64, cpu_usage: f64, network_usage: (u64, u64)) {
        static mut LAST_LOG_TIME: Option<std::time::Instant> = None;
        
        unsafe {
            let now = std::time::Instant::now();
            if let Some(last_time) = LAST_LOG_TIME {
                if last_time.elapsed().as_secs() >= 60 {
                    log_system_stats(memory_usage, cpu_usage, network_usage);
                    LAST_LOG_TIME = Some(now);
                }
            } else {
                LAST_LOG_TIME = Some(now);
            }
        }
    }
}
