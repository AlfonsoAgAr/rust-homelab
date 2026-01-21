use std::fs::{self, OpenOptions};
use std::io::Write;
use chrono::Local;

fn get_ram_kb(key: &str, text: &str) -> u64 {
    for line in text.lines() {
        if line.starts_with(key) {
           let parts: Vec<&str>= line .split_whitespace().collect();

           if parts.len() >= 2 {
                return parts[1].parse().unwrap_or(0);
           }
        }
    }
     0
}

fn main() {
    let path_csv = "/root/workspace/ram_history.csv";
    
    let content = fs::read_to_string("/proc/meminfo")
        .expect("Error reading /proc/meminfo");

    let total = get_ram_kb("MemTotal:", &content);
    let available = get_ram_kb("MemAvailable:", &content);
    let used = total - available;

    let percentage = (used as f64 / total as f64) / 100.0;
    let used_gb = used as f64 / 1024.0 / 1024.0;

    let date = Local::now().format("%Y-%m-%d %H-%M-%S");

    let exists = std::path::Path::new(path_csv).exists();

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path_csv)
        .expect("Error opening CSV");

    if !exists {
        writeln!(file, "timestamp,ram_usage_percentage,ram_used_gb").unwrap();
    }

    writeln!(file, "{},{:.2},{:.2}", date, percentage, used_gb)
        .expect("Error writing line");
}
