use std::collections::HashMap;
use sysinfo::{Components, Disks, Networks, System};

pub struct SystemInfo {
    pub system: System,
    pub components: Components,
    pub disks: Disks,
    pub networks: Networks,
}

impl SystemInfo {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        Self {
            system,
            components: Components::new_with_refreshed_list(),
            disks: Disks::new_with_refreshed_list(),
            networks: Networks::new_with_refreshed_list(),
        }
    }

    pub fn refresh_full(&mut self) {
        self.system.refresh_all();
        self.components.refresh();
        self.disks.refresh();
        self.networks.refresh();
    }

    pub fn refresh_light(&mut self) {
        use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind};
        self.system.refresh_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );
        self.components.refresh();
    }

    pub fn refresh_system_info(&mut self) {
        use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind};
        self.system.refresh_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );
        self.disks.refresh();
    }

    pub fn refresh_minimal(&mut self) {
        use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind};
        self.system.refresh_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );
    }

    // this bad boi right here does the refreshing all the time
    pub fn refresh(&mut self) {
        self.refresh_full();
    }

    pub fn get_cpu_usage(&self) -> f32 {
        self.system.global_cpu_usage()
    }

    pub fn get_memory_usage(&self) -> (u64, u64) {
        (self.system.used_memory(), self.system.total_memory())
    }

    pub fn get_memory_percentage(&self) -> f64 {
        let (used, total) = self.get_memory_usage();
        if total > 0 {
            (used as f64 / total as f64) * 100.0
        } else {
            0.0
        }
    }

    pub fn get_swap_usage(&self) -> (u64, u64) {
        (self.system.used_swap(), self.system.total_swap())
    }

    pub fn get_temperatures(&self) -> Vec<(String, f32)> {
        self.components
            .iter()
            .map(|component| (component.label().to_string(), component.temperature()))
            .collect()
    }

    pub fn get_system_info(&self) -> HashMap<String, String> {
        let mut info = HashMap::new();

        if let Some(name) = System::name() {
            info.insert("OS".to_string(), name);
        }
        if let Some(version) = System::os_version() {
            info.insert("Version".to_string(), version);
        }
        if let Some(kernel) = System::kernel_version() {
            info.insert("Kernel".to_string(), kernel);
        }
        if let Some(hostname) = System::host_name() {
            info.insert("Hostname".to_string(), hostname);
        }

        info.insert("CPUs".to_string(), self.system.cpus().len().to_string());
        if let Some(cpu) = self.system.cpus().first() {
            info.insert("CPU Brand".to_string(), cpu.brand().to_string());
        }

        info.insert(
            "Total Memory".to_string(),
            format_bytes(self.system.total_memory()),
        );

        info.insert("Uptime".to_string(), format_uptime(System::uptime()));

        info
    }

    pub fn get_processes(&self) -> Vec<ProcessInfo> {
        self.system
            .processes()
            .iter()
            .map(|(pid, process)| ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_string_lossy().to_string(),
                cpu_usage: process.cpu_usage(),
                memory: process.memory(),
                status: format!("{:?}", process.status()),
            })
            .collect()
    }

    pub fn get_disk_info(&self) -> Vec<DiskInfo> {
        self.disks
            .iter()
            .map(|disk| DiskInfo {
                name: disk.name().to_string_lossy().to_string(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                total_space: disk.total_space(),
                available_space: disk.available_space(),
            })
            .collect()
    }

    pub fn get_network_info(&self) -> Vec<(String, u64, u64)> {
        self.networks
            .iter()
            .map(|(name, network)| (name.to_string(), network.received(), network.transmitted()))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total_space: u64,
    pub available_space: u64,
}

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}

pub fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;

    if days > 0 {
        format!("{}d {}h {}m", days, hours, minutes)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}
