use std::fs;

pub fn get_os_name() {
    if let Ok(os_info) = fs::read_to_string("/etc/os-release") {
        
        let os_line = os_info
            .lines()
            .find(|line| line.starts_with("PRETTY_NAME="));

        match os_line {
            Some(line) => {
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() >= 2 {
                    let os_name = parts[1].trim().trim_matches('"');
                    println!("OS: {}", os_name);
                }
            }
            None => println!("OS: Linux Generic"),
        }
    } else {
        println!("Gagal membaca informasi OS!");
    }
}
