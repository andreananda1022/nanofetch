use nanofetch::info::{os, cpu, memory};

fn main() {
    os::get_os_name();
    cpu::get_cpu_name();
    memory::get_meminfo();
}
