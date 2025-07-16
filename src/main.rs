use colored::*;
use figlet_rs::FIGfont;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use std::{collections::HashMap, env, fs, path::Path, process::Command, thread, time::Duration};
use users::{get_current_uid, get_user_by_uid};

struct SystemInfo {
    os_name: String,
    os_version: String,
    kernel: String,
    hostname: String,
    username: String,
    shell: String,
    desktop: String,
    terminal: String,
    cpu_model: String,
    cpu_cores: usize,
    cpu_threads: usize,
    cpu_freq: String,
    cpu_temp: f32,
    memory_total: u64,
    memory_used: u64,
    memory_free: u64,
    memory_cached: u64,
    memory_buffers: u64,
    swap_total: u64,
    swap_used: u64,
    uptime: u64,
    load_avg: (f64, f64, f64),
    processes: usize,
    packages: HashMap<String, u32>,
    gpu_info: Vec<String>,
    disks: Vec<DiskInfo>,
    network: Vec<NetworkInfo>,
    battery: Option<BatteryInfo>,
    locale: String,
    timezone: String,
    ip_addresses: Vec<String>,
    architecture: String,
    virtualization: String,
    boot_time: u64,
    session_type: String,
    security_info: SecurityInfo,
}

#[derive(Debug)]
struct DiskInfo {
    name: String,
    mount_point: String,
    filesystem: String,
    size: u64,
    used: u64,
    usage_percent: f32,
}

#[derive(Debug)]
struct NetworkInfo {
    interface: String,
    mac: String,
    rx_bytes: u64,
    tx_bytes: u64,
}

#[derive(Debug)]
struct BatteryInfo {
    percentage: f32,
    status: String,
}

#[derive(Debug)]
struct SecurityInfo {
    selinux: String,
    apparmor: String,
    firewall: String,
}

impl SystemInfo {
    fn new() -> Self {
        SystemInfo {
            os_name: String::new(),
            os_version: String::new(),
            kernel: String::new(),
            hostname: String::new(),
            username: String::new(),
            shell: String::new(),
            desktop: String::new(),
            terminal: String::new(),
            cpu_model: String::new(),
            cpu_cores: 0,
            cpu_threads: 0,
            cpu_freq: String::new(),
            cpu_temp: 0.0,
            memory_total: 0,
            memory_used: 0,
            memory_free: 0,
            memory_cached: 0,
            memory_buffers: 0,
            swap_total: 0,
            swap_used: 0,
            uptime: 0,
            load_avg: (0.0, 0.0, 0.0),
            processes: 0,
            packages: HashMap::new(),
            gpu_info: Vec::new(),
            disks: Vec::new(),
            network: Vec::new(),
            battery: None,
            locale: String::new(),
            timezone: String::new(),
            ip_addresses: Vec::new(),
            architecture: String::new(),
            virtualization: String::new(),
            boot_time: 0,
            session_type: String::new(),
            security_info: SecurityInfo {
                selinux: String::new(),
                apparmor: String::new(),
                firewall: String::new(),
            },
        }
    }

    fn fetch_all(&mut self) {
        println!(
            "{}",
            "🚀 INITIALIZING ULTIMATE SYSTEM SCANNER..."
                .bright_cyan()
                .bold()
        );

        let pb = ProgressBar::new(10);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}",
                )
                .unwrap()
                .progress_chars("#>-"),
        );

        let tasks = vec![
            "OS Information",
            "Hardware Detection",
            "Memory Analysis",
            "Storage Scanning",
            "Network Discovery",
            "Security Audit",
            "Package Counting",
            "GPU Detection",
            "Battery Status",
            "Environment Setup",
        ];

        for task in tasks {
            pb.set_message(format!("Scanning {}", task));
            thread::sleep(Duration::from_millis(50)); // Simulate work
            pb.inc(1);
        }

        pb.finish_with_message("🎉 System scan complete!");

        // Now actually fetch the data
        self.fetch_os_info();
        self.fetch_hardware();
        self.fetch_memory();
        self.fetch_storage();
        self.fetch_network();
        self.fetch_security();
        self.fetch_packages();
        self.fetch_gpu();
        self.fetch_battery();
        self.fetch_environment();
    }

    fn fetch_os_info(&mut self) {
        // Enhanced OS detection
        if let Ok(contents) = fs::read_to_string("/etc/os-release") {
            let mut values = HashMap::new();
            for line in contents.lines() {
                if let Some((key, value)) = line.split_once('=') {
                    values.insert(key, value.trim_matches('"'));
                }
            }

            self.os_name = values.get("PRETTY_NAME").unwrap_or(&"Unknown").to_string();
            self.os_version = values.get("VERSION").unwrap_or(&"Unknown").to_string();
        }

        // Kernel info
        if let Ok(version) = fs::read_to_string("/proc/version") {
            self.kernel = version
                .split_whitespace()
                .take(3)
                .collect::<Vec<_>>()
                .join(" ");
        }

        // Hostname
        if let Ok(hostname) = fs::read_to_string("/etc/hostname") {
            self.hostname = hostname.trim().to_string();
        }

        // Architecture
        self.architecture = std::env::consts::ARCH.to_string();

        // Virtualization detection
        self.virtualization = detect_virtualization();

        // Boot time
        if let Ok(stat) = fs::read_to_string("/proc/stat") {
            for line in stat.lines() {
                if line.starts_with("btime") {
                    if let Some(boot_time_str) = line.split_whitespace().nth(1) {
                        self.boot_time = boot_time_str.parse().unwrap_or(0);
                        break;
                    }
                }
            }
        }

        // Uptime
        if let Ok(uptime_str) = fs::read_to_string("/proc/uptime") {
            if let Some(uptime_seconds) = uptime_str.split_whitespace().next() {
                self.uptime = uptime_seconds.parse::<f64>().unwrap_or(0.0) as u64;
            }
        }

        // Load average
        if let Ok(loadavg) = fs::read_to_string("/proc/loadavg") {
            let parts: Vec<&str> = loadavg.split_whitespace().collect();
            if parts.len() >= 3 {
                self.load_avg = (
                    parts[0].parse().unwrap_or(0.0),
                    parts[1].parse().unwrap_or(0.0),
                    parts[2].parse().unwrap_or(0.0),
                );
            }
        }
    }

    fn fetch_hardware(&mut self) {
        // Enhanced CPU info
        if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
            let mut core_count = 0;
            let mut thread_count = 0;

            for line in cpuinfo.lines() {
                if line.starts_with("model name") {
                    if self.cpu_model.is_empty() {
                        self.cpu_model = line
                            .split(':')
                            .nth(1)
                            .unwrap_or("Unknown")
                            .trim()
                            .to_string();
                    }
                } else if line.starts_with("cpu cores") {
                    core_count = line
                        .split(':')
                        .nth(1)
                        .unwrap_or("0")
                        .trim()
                        .parse()
                        .unwrap_or(0);
                } else if line.starts_with("processor") {
                    thread_count += 1;
                }
            }

            self.cpu_cores = core_count;
            self.cpu_threads = thread_count;
        }

        // CPU frequency
        if let Ok(freq) = fs::read_to_string("/proc/cpuinfo") {
            for line in freq.lines() {
                if line.starts_with("cpu MHz") {
                    if let Some(freq_str) = line.split(':').nth(1) {
                        if let Ok(freq_val) = freq_str.trim().parse::<f64>() {
                            self.cpu_freq = format!("{:.2} GHz", freq_val / 1000.0);
                            break;
                        }
                    }
                }
            }
        }

        // CPU temperature (if available)
        if let Ok(temp) = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp") {
            if let Ok(temp_val) = temp.trim().parse::<f32>() {
                self.cpu_temp = temp_val / 1000.0;
            }
        }

        // Process count
        if let Ok(entries) = fs::read_dir("/proc") {
            self.processes = entries
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        e.file_name()
                            .to_str()
                            .and_then(|name| name.parse::<u32>().ok())
                    })
                })
                .count();
        }
    }

    fn fetch_memory(&mut self) {
        if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
            let mut values = HashMap::new();

            for line in meminfo.lines() {
                if let Some((key, value)) = line.split_once(':') {
                    if let Some(num_str) = value.trim().split_whitespace().next() {
                        if let Ok(num) = num_str.parse::<u64>() {
                            values.insert(key.trim(), num * 1024); // Convert from kB to bytes
                        }
                    }
                }
            }

            self.memory_total = values.get("MemTotal").copied().unwrap_or(0);
            self.memory_free = values.get("MemFree").copied().unwrap_or(0);
            self.memory_cached = values.get("Cached").copied().unwrap_or(0);
            self.memory_buffers = values.get("Buffers").copied().unwrap_or(0);
            self.memory_used =
                self.memory_total - self.memory_free - self.memory_cached - self.memory_buffers;

            self.swap_total = values.get("SwapTotal").copied().unwrap_or(0);
            self.swap_used = self.swap_total - values.get("SwapFree").copied().unwrap_or(0);
        }
    }

    fn fetch_storage(&mut self) {
        // Parse /proc/mounts for mounted filesystems
        if let Ok(mounts) = fs::read_to_string("/proc/mounts") {
            for line in mounts.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let device = parts[0];
                    let mount_point = parts[1];
                    let filesystem = parts[2];

                    // Skip pseudo filesystems
                    if filesystem.starts_with("proc")
                        || filesystem.starts_with("sys")
                        || filesystem.starts_with("dev")
                        || filesystem.starts_with("tmp")
                    {
                        continue;
                    }

                    if let Ok(statvfs) = get_disk_usage(mount_point) {
                        let disk_info = DiskInfo {
                            name: device.to_string(),
                            mount_point: mount_point.to_string(),
                            filesystem: filesystem.to_string(),
                            size: statvfs.0,
                            used: statvfs.1,
                            usage_percent: (statvfs.1 as f32 / statvfs.0 as f32) * 100.0,
                        };
                        self.disks.push(disk_info);
                    }
                }
            }
        }
    }

    fn fetch_network(&mut self) {
        // Network interfaces
        if let Ok(interfaces) = fs::read_dir("/sys/class/net") {
            for interface in interfaces {
                if let Ok(interface) = interface {
                    let interface_name = interface.file_name().to_string_lossy().to_string();

                    if interface_name == "lo" {
                        continue;
                    } // Skip loopback

                    let mut net_info = NetworkInfo {
                        interface: interface_name.clone(),
                        mac: String::new(),
                        rx_bytes: 0,
                        tx_bytes: 0,
                    };

                    // Get MAC address
                    if let Ok(mac) =
                        fs::read_to_string(format!("/sys/class/net/{}/address", interface_name))
                    {
                        net_info.mac = mac.trim().to_string();
                    }

                    // Get RX/TX bytes
                    if let Ok(rx) = fs::read_to_string(format!(
                        "/sys/class/net/{}/statistics/rx_bytes",
                        interface_name
                    )) {
                        net_info.rx_bytes = rx.trim().parse().unwrap_or(0);
                    }
                    if let Ok(tx) = fs::read_to_string(format!(
                        "/sys/class/net/{}/statistics/tx_bytes",
                        interface_name
                    )) {
                        net_info.tx_bytes = tx.trim().parse().unwrap_or(0);
                    }

                    self.network.push(net_info);
                }
            }
        }

        // Get IP addresses
        if let Ok(output) = Command::new("ip").args(&["addr", "show"]).output() {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                let re = Regex::new(r"inet (\d+\.\d+\.\d+\.\d+)").unwrap();
                for cap in re.captures_iter(&output_str) {
                    let ip = cap[1].to_string();
                    if ip != "127.0.0.1" {
                        self.ip_addresses.push(ip);
                    }
                }
            }
        }
    }

    fn fetch_security(&mut self) {
        // SELinux status
        if let Ok(selinux) = fs::read_to_string("/sys/fs/selinux/enforce") {
            self.security_info.selinux = match selinux.trim() {
                "1" => "Enforcing".to_string(),
                "0" => "Permissive".to_string(),
                _ => "Disabled".to_string(),
            };
        } else {
            self.security_info.selinux = "Not Available".to_string();
        }

        // AppArmor status
        if Path::new("/sys/kernel/security/apparmor").exists() {
            self.security_info.apparmor = "Active".to_string();
        } else {
            self.security_info.apparmor = "Inactive".to_string();
        }

        // Firewall status
        if let Ok(output) = Command::new("ufw").arg("status").output() {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                self.security_info.firewall = if output_str.contains("active") {
                    "UFW Active".to_string()
                } else {
                    "UFW Inactive".to_string()
                };
            }
        } else {
            self.security_info.firewall = "Unknown".to_string();
        }
    }

    fn fetch_packages(&mut self) {
        // Package managers detection
        let package_managers = vec![
            ("apt", "dpkg -l 2>/dev/null | grep -c '^ii'"),
            ("yum", "rpm -qa 2>/dev/null | wc -l"),
            ("dnf", "rpm -qa 2>/dev/null | wc -l"),
            ("pacman", "pacman -Q 2>/dev/null | wc -l"),
            (
                "portage",
                "find /var/db/pkg -mindepth 2 -maxdepth 2 -type d 2>/dev/null | wc -l",
            ),
            ("snap", "snap list 2>/dev/null | tail -n +2 | wc -l"),
            ("flatpak", "flatpak list 2>/dev/null | wc -l"),
        ];

        for (manager, command) in package_managers {
            if let Ok(output) = Command::new("sh").arg("-c").arg(command).output() {
                if let Ok(count_str) = String::from_utf8(output.stdout) {
                    if let Ok(count) = count_str.trim().parse::<u32>() {
                        if count > 0 {
                            self.packages.insert(manager.to_string(), count);
                        }
                    }
                }
            }
        }
    }

    fn fetch_gpu(&mut self) {
        // NVIDIA GPU
        if let Ok(output) = Command::new("nvidia-smi")
            .args(&[
                "--query-gpu=gpu_name,memory.total,memory.used,temperature.gpu",
                "--format=csv,noheader,nounits",
            ])
            .output()
        {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                for line in output_str.lines() {
                    if !line.trim().is_empty() {
                        self.gpu_info.push(format!("NVIDIA: {}", line));
                    }
                }
            }
        }

        // AMD GPU via lspci
        if let Ok(output) = Command::new("lspci").output() {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                for line in output_str.lines() {
                    if (line.contains("VGA") || line.contains("Display")) && line.contains("AMD") {
                        self.gpu_info.push(format!(
                            "AMD: {}",
                            line.split(':').last().unwrap_or("Unknown").trim()
                        ));
                    }
                }
            }
        }

        // Generic GPU detection
        if self.gpu_info.is_empty() {
            if let Ok(output) = Command::new("lspci").output() {
                if let Ok(output_str) = String::from_utf8(output.stdout) {
                    for line in output_str.lines() {
                        if line.contains("VGA") || line.contains("Display") {
                            self.gpu_info.push(format!(
                                "GPU: {}",
                                line.split(':').last().unwrap_or("Unknown").trim()
                            ));
                        }
                    }
                }
            }
        }
    }

    fn fetch_battery(&mut self) {
        if let Ok(entries) = fs::read_dir("/sys/class/power_supply") {
            for entry in entries {
                if let Ok(entry) = entry {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.starts_with("BAT") {
                        let base_path = format!("/sys/class/power_supply/{}", name);

                        let mut battery = BatteryInfo {
                            percentage: 0.0,
                            status: String::new(),
                        };

                        if let Ok(capacity) = fs::read_to_string(format!("{}/capacity", base_path))
                        {
                            battery.percentage = capacity.trim().parse().unwrap_or(0.0);
                        }

                        if let Ok(status) = fs::read_to_string(format!("{}/status", base_path)) {
                            battery.status = status.trim().to_string();
                        }

                        self.battery = Some(battery);
                        break;
                    }
                }
            }
        }
    }

    fn fetch_environment(&mut self) {
        // User info
        if let Some(user) = get_user_by_uid(get_current_uid()) {
            self.username = user.name().to_string_lossy().to_string();
        }

        // Shell
        if let Ok(shell) = env::var("SHELL") {
            self.shell = shell.split('/').last().unwrap_or("Unknown").to_string();
        }

        // Desktop Environment
        if let Ok(desktop) = env::var("XDG_CURRENT_DESKTOP") {
            self.desktop = desktop;
        } else if let Ok(desktop) = env::var("DESKTOP_SESSION") {
            self.desktop = desktop;
        } else {
            self.desktop = "Unknown".to_string();
        }

        // Terminal
        if let Ok(term) = env::var("TERM") {
            self.terminal = term;
        }

        // Locale
        if let Ok(locale) = env::var("LANG") {
            self.locale = locale;
        }

        // Timezone
        if let Ok(tz) = fs::read_to_string("/etc/timezone") {
            self.timezone = tz.trim().to_string();
        }

        // Session type
        if let Ok(session) = env::var("XDG_SESSION_TYPE") {
            self.session_type = session;
        }
    }

    fn display(&self) {
        self.display_header();
        self.display_system_info();
        self.display_hardware_info();
        self.display_memory_info();
        self.display_storage_info();
        self.display_network_info();
        self.display_security_info();
        self.display_packages();
        self.display_environment();
        self.display_footer();
    }

    fn display_header(&self) {
        println!("\n{}", "═".repeat(80).bright_blue());

        // Fancy figlet header
        if let Ok(font) = FIGfont::standard() {
            if let Some(figure) = font.convert("SYSINFO") {
                println!("{}", figure.to_string().bright_cyan().bold());
            }
        }

        println!(
            "{}",
            "🚀 ULTIMATE SYSTEM INFORMATION SCANNER v2.0"
                .bright_yellow()
                .bold()
        );
        println!(
            "{}",
            format!(
                "📊 Scanned at: {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
            )
            .bright_green()
        );
        println!("{}", "═".repeat(80).bright_blue());
    }

    fn display_system_info(&self) {
        println!("\n{}", "🖥️  SYSTEM INFORMATION".bright_magenta().bold());
        println!("{}", "─".repeat(50).bright_blue());

        self.print_info("", "OS Name", &self.os_name);
        self.print_info("", "OS Version", &self.os_version);
        self.print_info("", "Kernel", &self.kernel);
        self.print_info("", "Architecture", &self.architecture);
        self.print_info("", "Hostname", &self.hostname);
        self.print_info("⏰", "Uptime", &format_duration(self.uptime));
        self.print_info(
            "📈",
            "Load Average",
            &format!(
                "{:.2} {:.2} {:.2}",
                self.load_avg.0, self.load_avg.1, self.load_avg.2
            ),
        );
        self.print_info("🔧", "Processes", &self.processes.to_string());

        if !self.virtualization.is_empty() {
            self.print_info("🔮", "Virtualization", &self.virtualization);
        }
    }

    fn display_hardware_info(&self) {
        println!("\n{}", "⚡ HARDWARE INFORMATION".bright_magenta().bold());
        println!("{}", "─".repeat(50).bright_blue());

        self.print_info("🖨️", "CPU Model", &self.cpu_model);
        self.print_info(
            "🔢",
            "CPU Cores",
            &format!("{} cores / {} threads", self.cpu_cores, self.cpu_threads),
        );
        self.print_info("⚡", "CPU Frequency", &self.cpu_freq);

        if self.cpu_temp > 0.0 {
            let temp_indicator = if self.cpu_temp > 80.0 {
                "🔥"
            } else if self.cpu_temp > 60.0 {
                "🌡️"
            } else {
                "❄️"
            };
            self.print_info(
                temp_indicator,
                "CPU Temperature",
                &format!("{:.1}°C", self.cpu_temp),
            );
        }

        for gpu in &self.gpu_info {
            self.print_info("🎮", "GPU", gpu);
        }
    }

    fn display_memory_info(&self) {
        println!("\n{}", "💾 MEMORY INFORMATION".bright_magenta().bold());
        println!("{}", "─".repeat(50).bright_blue());

        let memory_used_gb = self.memory_used as f64 / 1024.0 / 1024.0 / 1024.0;
        let memory_total_gb = self.memory_total as f64 / 1024.0 / 1024.0 / 1024.0;
        let memory_percent = (self.memory_used as f64 / self.memory_total as f64) * 100.0;

        self.print_info(
            "📊",
            "Memory Usage",
            &format!(
                "{:.2} GB / {:.2} GB ({:.1}%)",
                memory_used_gb, memory_total_gb, memory_percent
            ),
        );
        self.print_memory_bar(memory_percent);

        let cached_gb = self.memory_cached as f64 / 1024.0 / 1024.0 / 1024.0;
        let buffers_gb = self.memory_buffers as f64 / 1024.0 / 1024.0 / 1024.0;
        self.print_info("📝", "Cached", &format!("{:.2} GB", cached_gb));
        self.print_info("🔄", "Buffers", &format!("{:.2} GB", buffers_gb));

        if self.swap_total > 0 {
            let swap_used_gb = self.swap_used as f64 / 1024.0 / 1024.0 / 1024.0;
            let swap_total_gb = self.swap_total as f64 / 1024.0 / 1024.0 / 1024.0;
            let swap_percent = (self.swap_used as f64 / self.swap_total as f64) * 100.0;
            self.print_info(
                "🔄",
                "Swap Usage",
                &format!(
                    "{:.2} GB / {:.2} GB ({:.1}%)",
                    swap_used_gb, swap_total_gb, swap_percent
                ),
            );
        }
    }

    fn display_storage_info(&self) {
        println!("\n{}", "💽 STORAGE INFORMATION".bright_magenta().bold());
        println!("{}", "─".repeat(50).bright_blue());

        for disk in &self.disks {
            let size_gb = disk.size as f64 / 1024.0 / 1024.0 / 1024.0;
            let used_gb = disk.used as f64 / 1024.0 / 1024.0 / 1024.0;

            println!(
                "  {} {}",
                "💾".bright_yellow(),
                disk.mount_point.bright_white().bold()
            );
            println!(
                "    {} {}",
                "Device:".bright_blue(),
                disk.name.bright_cyan()
            );
            println!(
                "    {} {}",
                "Filesystem:".bright_blue(),
                disk.filesystem.bright_cyan()
            );
            println!(
                "    {} {:.2} GB / {:.2} GB ({:.1}%)",
                "Usage:".bright_blue(),
                used_gb,
                size_gb,
                disk.usage_percent
            );
            self.print_disk_bar(disk.usage_percent);
            println!();
        }
    }

    fn display_network_info(&self) {
        println!("\n{}", "🌐 NETWORK INFORMATION".bright_magenta().bold());
        println!("{}", "─".repeat(50).bright_blue());

        for ip in &self.ip_addresses {
            self.print_info("🌍", "IP Address", ip);
        }

        for net in &self.network {
            println!(
                "  {} {}",
                "📡".bright_yellow(),
                net.interface.bright_white().bold()
            );
            println!("    {} {}", "MAC:".bright_blue(), net.mac.bright_cyan());
            println!(
                "    {} {}",
                "RX:".bright_blue(),
                format_bytes(net.rx_bytes).bright_green()
            );
            println!(
                "    {} {}",
                "TX:".bright_blue(),
                format_bytes(net.tx_bytes).bright_red()
            );
            println!();
        }
    }

    fn display_security_info(&self) {
        println!("\n{}", "🔐 SECURITY INFORMATION".bright_magenta().bold());
        println!("{}", "─".repeat(50).bright_blue());

        self.print_info("🛡️", "SELinux", &self.security_info.selinux);
        self.print_info("🔒", "AppArmor", &self.security_info.apparmor);
        self.print_info("🔥", "Firewall", &self.security_info.firewall);
    }

    fn display_packages(&self) {
        println!("\n{}", "📦 PACKAGE INFORMATION".bright_magenta().bold());
        println!("{}", "─".repeat(50).bright_blue());

        for (manager, count) in &self.packages {
            self.print_info(
                "📦",
                &format!("{} packages", manager.to_uppercase()),
                &count.to_string(),
            );
        }
    }

    fn display_environment(&self) {
        println!("\n{}", "🏠 ENVIRONMENT INFORMATION".bright_magenta().bold());
        println!("{}", "─".repeat(50).bright_blue());

        self.print_info("👤", "Username", &self.username);
        self.print_info("🐚", "Shell", &self.shell);
        self.print_info("🖥️", "Desktop", &self.desktop);
        self.print_info("💻", "Terminal", &self.terminal);
        self.print_info("🌍", "Locale", &self.locale);
        self.print_info("🕐", "Timezone", &self.timezone);
        self.print_info("🎭", "Session", &self.session_type);

        if let Some(battery) = &self.battery {
            self.print_info(
                "🔋",
                "Battery",
                &format!("{}% ({})", battery.percentage, battery.status),
            );
            self.print_battery_bar(battery.percentage);
        }
    }

    fn display_footer(&self) {
        println!("\n{}", "═".repeat(80).bright_blue());
        println!("{}", "🎉 SYSTEM SCAN COMPLETE! 🎉".bright_green().bold());
        println!(
            "{}",
            "Generated by the most INSANE system info tool ever created!"
                .bright_yellow()
                .italic()
        );
        println!("{}", "═".repeat(80).bright_blue());
    }

    fn print_info(&self, icon: &str, label: &str, value: &str) {
        println!(
            "  {} {:<20} {}",
            icon.bright_yellow(),
            format!("{}:", label).bright_blue().bold(),
            value.bright_white().italic()
        );
    }

    fn print_memory_bar(&self, percentage: f64) {
        let bar_width = 40;
        let filled = (percentage / 100.0 * bar_width as f64) as usize;
        let empty = bar_width - filled;

        let bar = format!(
            "[{}{}]",
            "█".repeat(filled).bright_green(),
            "░".repeat(empty).bright_black()
        );

        println!("    {}", bar);
    }

    fn print_disk_bar(&self, percentage: f32) {
        let bar_width = 40;
        let filled = (percentage / 100.0 * bar_width as f32) as usize;
        let empty = bar_width - filled;

        let bar_color = if percentage > 90.0 {
            "█".repeat(filled).bright_red()
        } else if percentage > 70.0 {
            "█".repeat(filled).bright_yellow()
        } else {
            "█".repeat(filled).bright_green()
        };

        let bar = format!("[{}{}]", bar_color, "░".repeat(empty).bright_black());

        println!("    {}", bar);
    }

    fn print_battery_bar(&self, percentage: f32) {
        let bar_width = 20;
        let filled = (percentage / 100.0 * bar_width as f32) as usize;
        let empty = bar_width - filled;

        let bar_color = if percentage > 60.0 {
            "█".repeat(filled).bright_green()
        } else if percentage > 20.0 {
            "█".repeat(filled).bright_yellow()
        } else {
            "█".repeat(filled).bright_red()
        };

        let bar = format!("[{}{}]", bar_color, "░".repeat(empty).bright_black());

        println!("    {}", bar);
    }
}

fn main() {
    // Clear screen for maximum impact
    print!("\x1B[2J\x1B[1;1H");

    let mut sysinfo = SystemInfo::new();
    sysinfo.fetch_all();
    sysinfo.display();
}

// Helper functions
fn detect_virtualization() -> String {
    if let Ok(output) = Command::new("systemd-detect-virt").output() {
        if let Ok(virt) = String::from_utf8(output.stdout) {
            let virt = virt.trim();
            if virt != "none" {
                return virt.to_string();
            }
        }
    }

    // Check for common virtualization indicators
    if Path::new("/proc/xen").exists() {
        return "Xen".to_string();
    }

    if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
        if cpuinfo.contains("QEMU") {
            return "QEMU".to_string();
        }
        if cpuinfo.contains("VMware") {
            return "VMware".to_string();
        }
    }

    String::new()
}

fn get_disk_usage(path: &str) -> Result<(u64, u64, u64), std::io::Error> {
    use std::ffi::CString;
    use std::mem;

    let path_c = CString::new(path)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid path"))?;
    let mut statvfs: libc::statvfs = unsafe { mem::zeroed() };

    let result = unsafe { libc::statvfs(path_c.as_ptr(), &mut statvfs) };

    if result == 0 {
        let block_size = statvfs.f_frsize as u64;
        let total_blocks = statvfs.f_blocks as u64;
        let free_blocks = statvfs.f_bavail as u64;

        let total_size = total_blocks * block_size;
        let available_size = free_blocks * block_size;
        let used_size = total_size - available_size;

        Ok((total_size, used_size, available_size))
    } else {
        Err(std::io::Error::last_os_error())
    }
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}

fn format_duration(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;

    if days > 0 {
        format!("{} days, {} hours, {} minutes", days, hours, minutes)
    } else if hours > 0 {
        format!("{} hours, {} minutes", hours, minutes)
    } else {
        format!("{} minutes", minutes)
    }
}
