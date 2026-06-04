use std::fs;

pub fn get_meminfo() {
    let meminfo = fs::read_to_string("/proc/meminfo").expect("Gagal membaca /proc/meminfo");
    let mut lines = meminfo.lines();

    let memtotal_line = lines.nth(0).unwrap_or("");
    let memavailable_line = lines.nth(1).unwrap_or("");

    let memtotal: u64 = memtotal_line
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let memavailable: u64 = memavailable_line
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let memusage = memtotal - memavailable;

    println!("Memory: {:.2}MiB / {:.2}MiB", memusage / 1024, memtotal / 1024);
}
