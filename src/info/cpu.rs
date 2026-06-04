use std::fs;

pub fn get_cpu_name() {
    let cpuinfo = fs::read_to_string("/proc/cpuinfo").expect("Gagal membaca /proc/cpuinfo");
    let cpu_name_line = cpuinfo.lines().find(|line| line.starts_with("model name"));

    match cpu_name_line {
        Some(line) => {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 {
                println!("CPU: {}", parts[1].trim());
            }
        }
        None => println!("Informasi nama CPU tidak ditemukan!"),
    }
}
