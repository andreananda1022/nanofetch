use std::fs;
use std::env;

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
        "Kernel: 6.5.0-14-generic".to_string(),
        "Uptime: 2h 15m".to_string(),
        "Packages: 1234 (dpkg)".to_string(),
        "Memory: 1024.00MiB / 4096.00MiB".to_string(),
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
