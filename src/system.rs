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
        self.components.refresh(true);
        self.disks.refresh(true);
        self.networks.refresh(true);
    }

    pub fn refresh_light(&mut self) {
        use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind};
        self.system.refresh_specifics(
            RefreshKind::nothing()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );
        self.components.refresh(true);
    }

    pub fn refresh_system_info(&mut self) {
        use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind};
        self.system.refresh_specifics(
            RefreshKind::nothing()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );
        self.disks.refresh(true);
    }

    pub fn refresh_minimal(&mut self) {
        use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind};
        self.system.refresh_specifics(
            RefreshKind::nothing()
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

    pub fn get_temperatures(&self) -> Vec<(String, f32)> {
        self.components
            .iter()
            .filter_map(|component| {
                component
                    .temperature()
                    .map(|temp| (component.label().to_string(), temp))
            })
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
            .map(|(pid, process)| {
                let name = process.name().to_string_lossy().to_string();
                let category = categorize_process(&name);

                ProcessInfo {
                    pid: pid.as_u32(),
                    name,
                    cpu_usage: process.cpu_usage(),
                    memory: process.memory(),
                    status: format!("{:?}", process.status()),
                    category,
                }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProcessCategory {
    System,
    Browser,
    Development,
    Terminal,
    Editor,
    Media,
    Background,
    User,
}

impl ProcessCategory {
    pub fn name(&self) -> &str {
        match self {
            ProcessCategory::System => "System",
            ProcessCategory::Browser => "Browser",
            ProcessCategory::Development => "Development",
            ProcessCategory::Terminal => "Terminal",
            ProcessCategory::Editor => "Editor",
            ProcessCategory::Media => "Media",
            ProcessCategory::Background => "Background",
            ProcessCategory::User => "User",
        }
    }

    pub fn color(&self) -> ratatui::style::Color {
        use ratatui::style::Color;
        match self {
            ProcessCategory::System => Color::Red,
            ProcessCategory::Browser => Color::Cyan,
            ProcessCategory::Development => Color::Magenta,
            ProcessCategory::Terminal => Color::Green,
            ProcessCategory::Editor => Color::Yellow,
            ProcessCategory::Media => Color::Blue,
            ProcessCategory::Background => Color::Gray,
            ProcessCategory::User => Color::White,
        }
    }
}

pub fn categorize_process(name: &str) -> ProcessCategory {
    let name_lower = name.to_lowercase();

    if name_lower.contains("systemd")
        || name_lower.contains("kernel")
        || name_lower.starts_with("kworker")
        || name_lower.starts_with("ksoftirqd")
        || name_lower.starts_with("migration")
        || name_lower.starts_with("rcu")
        || name_lower.starts_with("watchdog")
        || name_lower.contains("dbus")
        || name_lower.contains("udev")
        || name_lower.contains("polkit")
        || name_lower.contains("NetworkManager")
        || name_lower.contains("bluetooth")
        || name_lower == "init"
        || name_lower == "kthreadd"
    {
        return ProcessCategory::System;
    }

    if name_lower.contains("firefox")
        || name_lower.contains("chrome")
        || name_lower.contains("chromium")
        || name_lower.contains("brave")
        || name_lower.contains("edge")
        || name_lower.contains("safari")
        || name_lower.contains("opera")
        || name_lower.contains("vivaldi")
    {
        return ProcessCategory::Browser;
    }

    if name_lower.contains("rust")
        || name_lower.contains("cargo")
        || name_lower.contains("rustc")
        || name_lower.contains("gcc")
        || name_lower.contains("g++")
        || name_lower.contains("clang")
        || name_lower.contains("python")
        || name_lower.contains("node")
        || name_lower.contains("npm")
        || name_lower.contains("yarn")
        || name_lower.contains("java")
        || name_lower.contains("mvn")
        || name_lower.contains("gradle")
        || name_lower.contains("docker")
        || name_lower.contains("podman")
        || name_lower.contains("git")
    {
        return ProcessCategory::Development;
    }

    if name_lower.contains("terminal")
        || name_lower.contains("konsole")
        || name_lower.contains("gnome-terminal")
        || name_lower.contains("xterm")
        || name_lower.contains("alacritty")
        || name_lower.contains("kitty")
        || name_lower.contains("wezterm")
        || name_lower.contains("terminator")
        || name_lower == "bash"
        || name_lower == "zsh"
        || name_lower == "fish"
        || name_lower == "sh"
    {
        return ProcessCategory::Terminal;
    }

    if name_lower.contains("vim")
        || name_lower.contains("nvim")
        || name_lower.contains("neovim")
        || name_lower.contains("emacs")
        || name_lower.contains("code")
        || name_lower.contains("vscode")
        || name_lower.contains("sublime")
        || name_lower.contains("atom")
        || name_lower.contains("nano")
        || name_lower.contains("gedit")
        || name_lower.contains("kate")
    {
        return ProcessCategory::Editor;
    }

    if name_lower.contains("vlc")
        || name_lower.contains("mpv")
        || name_lower.contains("spotify")
        || name_lower.contains("rhythmbox")
        || name_lower.contains("totem")
        || name_lower.contains("ffmpeg")
        || name_lower.contains("pulseaudio")
        || name_lower.contains("pipewire")
        || name_lower.contains("alsa")
    {
        return ProcessCategory::Media;
    }

    if name_lower.ends_with("d")
        || name_lower.contains("daemon")
        || name_lower.contains("service")
        || name_lower.starts_with("gvfs")
    {
        return ProcessCategory::Background;
    }

    ProcessCategory::User
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
    pub status: String,
    pub category: ProcessCategory,
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
