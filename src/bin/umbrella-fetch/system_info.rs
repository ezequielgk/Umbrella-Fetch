//! System and hardware information collection.

use sysinfo::{System, Disks};
use std::fs;

/// Consolidated system telemetry data.
pub struct SystemInfo {
    pub user: String,
    pub hostname: String,
    pub os: String,
    pub kernel: String,
    pub uptime: String,
    pub cpu_model: String,
    pub cpu_cores: usize,
    pub ram_used_gb: f64,
    pub ram_total_gb: f64,
    pub swap_used_gb: f64,
    pub swap_total_gb: f64,
    pub shell: String,
    pub pkgs: String,
    pub display: String,
    pub wm: String,
    pub gpu_info: String,
    pub partitions: Vec<(String, u64, u64)>, // (Mount point, Used GB, Total GB)
    pub net_ip: String,
    pub net_tx_mb: f64,
    pub net_rx_mb: f64,
    pub net_ping: String,
    pub vpn_active: bool,
}

impl SystemInfo {
    /// Gathers all system information in a single call.
    pub fn fetch() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        
        let disks = Disks::new_with_refreshed_list();
        let mut partitions = Vec::new();
        for disk in disks.iter().take(3) {
            let mount = disk.mount_point().to_string_lossy().to_string();
            let total = disk.total_space() / 1_073_741_824;
            let used = total.saturating_sub(disk.available_space() / 1_073_741_824);
            partitions.push((mount, used, total));
        }
        
        let user = whoami::fallible::username().unwrap_or_else(|_| "Unknown".to_string());
        let hostname = whoami::fallible::hostname().unwrap_or_else(|_| "Unknown".to_string());
        let os = System::name().unwrap_or_else(|| "Unknown".to_string());
        let kernel = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
        
        let up = System::uptime();
        let uptime = format!("{}h {}m", up / 3600, (up % 3600) / 60);
        
        let cpu_cores = sys.cpus().len();
        let cpu_model = if let Some(cpu) = sys.cpus().first() {
            cpu.brand().to_string()
        } else {
            "Unknown".to_string()
        };
        
        let ram_used_gb = sys.used_memory() as f64 / 1_073_741_824.0;
        let ram_total_gb = sys.total_memory() as f64 / 1_073_741_824.0;
        let swap_used_gb = sys.used_swap() as f64 / 1_073_741_824.0;
        let swap_total_gb = sys.total_swap() as f64 / 1_073_741_824.0;
        
        let shell = std::env::var("SHELL")
            .unwrap_or_else(|_| "/bin/sh".to_string())
            .split('/')
            .last()
            .unwrap_or("unknown")
            .to_string();
            
        let wm = std::env::var("XDG_SESSION_DESKTOP").unwrap_or_else(|_| "Unknown".to_string());
        let display = std::env::var("WAYLAND_DISPLAY")
            .or_else(|_| std::env::var("DISPLAY"))
            .unwrap_or_else(|_| "TTY".to_string());
            
        let pkgs = {
            let mut result = "Unknown".to_string();
            
            // Arch Linux (pacman)
            if let Ok(entries) = fs::read_dir("/var/lib/pacman/local") {
                let count = entries.filter_map(Result::ok).filter(|e| e.file_type().map_or(false, |ft| ft.is_dir())).count();
                if count > 0 { result = format!("{} (pacman)", count); }
            }
            
            // Debian/Ubuntu (dpkg)
            if result == "Unknown" {
                if let Ok(entries) = fs::read_dir("/var/lib/dpkg/info") {
                    let count = entries.filter_map(Result::ok).filter(|e| e.path().extension().map_or(false, |ext| ext == "list")).count();
                    if count > 0 { result = format!("{} (dpkg)", count); }
                }
            }
            
            // Void Linux (xbps)
            if result == "Unknown" && std::process::Command::new("xbps-query").arg("-V").output().is_ok() {
                if let Ok(output) = std::process::Command::new("sh").arg("-c").arg("xbps-query -l | wc -l").output() {
                    let count_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if let Ok(count) = count_str.parse::<usize>() {
                        if count > 0 { result = format!("{} (xbps)", count); }
                    }
                }
            }
            
            // Fedora/RedHat (rpm)
            if result == "Unknown" && std::process::Command::new("rpm").arg("--version").output().is_ok() {
                if let Ok(output) = std::process::Command::new("sh").arg("-c").arg("rpm -qa | wc -l").output() {
                    let count_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if let Ok(count) = count_str.parse::<usize>() {
                        if count > 0 { result = format!("{} (rpm)", count); }
                    }
                }
            }
            
            result
        };
        
        let mut gpu_info = "Unknown".to_string();
        if let Ok(output) = std::process::Command::new("sh").arg("-c").arg("lspci | grep -i 'vga\\|3d\\|display' | cut -d ':' -f 3 | sed 's/^ *//' | head -n 1").output() {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if !stdout.trim().is_empty() {
                    gpu_info = stdout.trim().to_string();
                }
            }
        }

        let networks = sysinfo::Networks::new_with_refreshed_list();
        
        let mut net_ip = "127.0.0.1".to_string();
        let mut net_tx_mb = 0.0;
        let mut net_rx_mb = 0.0;
        let mut vpn_active = false;
        
        for (interface_name, data) in &networks {
            if interface_name.starts_with("tun") || interface_name.starts_with("wg") || interface_name.starts_with("tailscale") {
                vpn_active = true;
            }
            if interface_name.starts_with("en") || interface_name.starts_with("eth") || interface_name.starts_with("wl") || interface_name.starts_with("wlan") {
                net_tx_mb += data.transmitted() as f64 / 1_048_576.0;
                net_rx_mb += data.received() as f64 / 1_048_576.0;
            }
        }
        
        if let Ok(output) = std::process::Command::new("sh").arg("-c").arg("hostname -I | awk '{print $1}'").output() {
            let ip = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !ip.is_empty() {
                net_ip = ip;
            }
        }
        
        let mut net_ping = "TIMEOUT".to_string();
        if let Ok(output) = std::process::Command::new("sh").arg("-c").arg("ping -c1 -W1 192.168.1.1 | grep 'time=' | awk -F'time=' '{print $2}' | awk '{print $1 \" ms\"}'").output() {
            let ping = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !ping.is_empty() {
                net_ping = ping;
            }
        }

        Self {
            user,
            hostname,
            os,
            kernel,
            uptime,
            cpu_model,
            cpu_cores,
            ram_used_gb,
            ram_total_gb,
            swap_used_gb,
            swap_total_gb,
            shell,
            pkgs,
            display,
            wm,
            gpu_info,
            partitions,
            net_ip,
            net_tx_mb,
            net_rx_mb,
            net_ping,
            vpn_active,
        }
    }
}
