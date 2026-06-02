use std::fs;
use std::thread;
use std::time::Duration;

#[derive(Debug, Default)]
struct CpuStats {
    user: u64,
    nice: u64,
    system: u64,
    idle: u64,
    iowait: u64,
    irq: u64,
    softirq: u64,
}

fn get_cpu_stats() -> CpuStats {
    let content = fs::read_to_string("/proc/stat").expect("Gagal membaca /proc/stat");
    let line = content.lines().next().unwrap();
    let parts: Vec<u64> = line.split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    CpuStats {
        user: parts[0],
        nice: parts[1],
        system: parts[2],
        idle: parts[3],
        iowait: parts[4],
        irq: parts[5],
        softirq: parts[6],
    }
}

fn main() {
    let start = get_cpu_stats();

    println!("[tinyfetch]");
    thread::sleep(Duration::from_secs(1));

    let end = get_cpu_stats();

    let idle_diff = (end.idle + end.iowait) - (start.idle + start.iowait);
    let total_start = start.user + start.nice + start.system + start.idle + start.iowait + start.irq + start.softirq;
    let total_end = end.user + end.nice + end.system + end.idle + end.iowait + end.irq + end.softirq;
    let total_diff = total_end - total_start;

    let usage = 100.0 * (1.0 - (idle_diff as f64 / total_diff as f64));
    println!("Penggunaan CPU saat ini: {:.2}%", usage);
}
