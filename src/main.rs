use std::fs;
use std::env;
use std::process;

fn get_username() -> String {
    env::var("USER").unwrap_or_else(|_| String::from("user"))
}

fn get_hostname() -> String {
    fs::read_to_string("/etc/hostname")
        .unwrap_or_else(|_| String::from("hostname"))
        .trim()
        .to_string()
}

fn get_os_name() -> String {
    let content = fs::read_to_string("/etc/os-release").unwrap_or_else(|_| String::new());

    let os_name = content
        .lines()
        .find_map(|line| {
            line.strip_prefix("PRETTY_NAME=")
                .map(|s| s.trim_matches('"'))
        })
        .unwrap_or("Linux Generic")
        .to_string();

    format!("OS: {}", os_name)
}

fn get_device_model() -> String {
    let vendor = fs::read_to_string("/sys/class/dmi/id/sys_vendor")
        .map(|s| s.trim().to_string())
        .unwrap_or_default();

    let product = fs::read_to_string("/sys/class/dmi/id/product_name")
        .map(|s| s.trim().to_string())
        .unwrap_or_default();

    if vendor.is_empty() && product.is_empty() {
        String::from("Host: Desktop PC / Virtual Machine")
    } else {
        format!("Host: {} {}", vendor, product)
    }
}

fn get_kernel_info() -> String {
    let os_release = fs::read_to_string("/proc/sys/kernel/osrelease")
        .unwrap_or_else(|_| String::from("Unknown"))
        .trim()
        .to_string();


    format!("Kernel: {}", os_release)
}

fn get_uptime() -> String {
    let uptime_content = fs::read_to_string("/proc/uptime")
        .unwrap_or_else(|_| String::from("0.0 0.0"));

    let total_seconds: u64 = uptime_content
        .split_whitespace()
        .next()
        .and_then(|s| s.parse::<f64>().ok())
        .map(|seconds| seconds as u64)
        .unwrap_or(0);

    let days = total_seconds / 86400;
    let hours = (total_seconds % 86400) / 3600;
    let minutes = (total_seconds % 3600) / 60;

    if days > 0 {
        format!("Uptime: {} days, {} hours, {} mins", days, hours, minutes)
    } else if hours > 0 {
        format!("Uptime: {} hours, {} mins", hours, minutes)
    } else {
        format!("Uptime: {} mins", minutes)
    }
}

fn get_package_count() -> String {
    // Cek untuk Arch Linux (Pacman)
    if let Ok(entries) = fs::read_dir("/var/lib/pacman/local") {
        let count = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .count();

        format!("Package: {} (pacman)", count)

    // Cek untuk Debian/Ubuntu (Dpkg/APT)
    } else if let Ok(entries) = fs::read_dir("/var/lib/dpkg/info") {
        let count = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "list"))
            .count();

        format!("Package: {} (dpkg)", count)
        
    } else {
        String::from("Package: NaN")
    }
}

fn get_meminfo() -> String {
    let Ok(meminfo) = fs::read_to_string("/proc/meminfo") else {
        return "Memory: unavailable".to_string();
    };

    let mut total = None;
    let mut available = None;

    for line in meminfo.lines() {
        match () {
            _ if line.starts_with("MemTotal:") => {
                total = line.split_whitespace().nth(1)
                    .and_then(|s| s.parse::<u64>().ok());
            }
            _ if line.starts_with("MemAvailable:") => {
                available = line.split_whitespace().nth(1)
                    .and_then(|s| s.parse::<u64>().ok());
            }
            _ => {}
        }

        if total.is_some() && available.is_some() {
            break;
        }
    }

    match (total, available) {
        (Some(total), Some(avail)) => {
            let used = total.saturating_sub(avail) / 1024;
            format!("Memory: {}MiB / {}MiB", used, total / 1024)
        }
        _ => String::from("Memory: unavailable")
    }
}

fn get_ppid(pid: u32) -> Option<u32> {
    let stat = fs::read_to_string(format!("/proc/{}/stat", pid)).ok()?;

    let fields: Vec<&str> = stat.split_whitespace().collect();

    fields.get(3)?.parse().ok()
}

fn get_process_name(pid: u32) -> Option<String> {
    let comm = fs::read_to_string(format!("/proc/{}/comm", pid)).ok()?;

    Some(comm.trim().to_string())
}

fn is_shell(name: &str) -> bool {
    matches!(
        name,
        "bash" | "zsh" | "fish" | "dash" | "ksh" | "tcsh" | "csh" | "nu" | "xonsh"
    )
}

fn get_shell_name() -> String {
    let mut pid = process::id();

    while let Some(ppid) = get_ppid(pid) {
        let Some(name) = get_process_name(ppid) else {
            return String::from("Shell: sh");
        };

        if is_shell(&name) {
            return format!("Shell: {}", name);
        }

        if ppid <= 1 {
            break;
        }

        pid = ppid;
    }

    String::from("Shell: sh")
}

fn get_cpu_model() -> String {
    let Ok(cpuinfo) = std::fs::read_to_string("/proc/cpuinfo") else {
        return String::from("CPU: unknown");
    };

    cpuinfo
        .lines()
        .find(|line| line.starts_with("model name"))
        .and_then(|line| line.split_once(':'))
        .map(|(_, model)| format!("CPU: {}", model.trim()))
        .unwrap_or_else(|| String::from("CPU: unknown"))
}

fn get_gpu_info() -> String {
    let output = match process::Command::new("lspci").output() {
        Ok(result) => result,
        Err(_) => return String::from("GPU: tidak terdeteksi"),
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut gpu_info = Vec::new();

    for line in stdout.lines() {
        let line_lower = line.to_lowercase();
        if line_lower.contains("vga compatible controller") || line_lower.contains("3d controller") {
            let parts: Vec<&str> = line.splitn(2, ": ").collect();

            if parts.len() == 2 {
                gpu_info.push(parts[1].trim().to_string());
            }
        }
    }

    if gpu_info.is_empty() {
        String::from("GPU: GPU tidak terdeteksi")
    } else {
        format!("GPU: {}", gpu_info.join(" | "))
    }
}

fn main() {
    let ascii_raw = fs::read_to_string("src/logo.txt")
        .unwrap_or_else(|_| String::from("     ???   \n   No Logo \n     ???   "));
    
    let ascii_art: Vec<&str> = ascii_raw.lines().collect();
    let ascii_blank = "           ";

    let header = format!("{}@{}", get_username(), get_hostname());
    let header_length = header.chars().count();
    let constraint = "-".repeat(header_length);

    let info = vec![
        header,
        constraint,
        get_os_name(),
        get_device_model(),
        get_kernel_info(),
        get_uptime(),
        get_package_count(),
        get_shell_name(),
        get_cpu_model(),
        get_gpu_info(),
        get_meminfo()
    ];

    let max_lines = std::cmp::max(ascii_art.len(), info.len());

    for i in 0..max_lines {
        let ascii_line = if i < ascii_art.len() {
            ascii_art[i]
        } else {
            ascii_blank
        };

        let info_line = if i < info.len() {
            &info[i]
        } else {
            ""
        };

        println!("{}    {}", ascii_line, info_line);
    }
}
