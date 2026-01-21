use std::fs;
use std::thread;
use std::time::Duration;


fn get_ram_kb(key: &str, text: &str) -> u64 {
    for line in text.lines() {
        if line.starts_with(key) {
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() >= 2 {
                return parts[1].parse().unwrap_or(0);
            }
        }
    }
    0 // if key is missing
}

fn main() {
    println!("Loading RAM monitor");

    loop {
        let content = fs::read_to_string("/proc/meminfo")
            .expect("ERROR reading meminfo");

        let total = get_ram_kb("MemTotal:", &content);
        let available = get_ram_kb("MemAvailable:", &content);

        let used = total - available;
        let percentage = (used as f64 / total as f64) * 100.0;

        let total_gb = total as f64 / 1024.0 / 1024.0;
        let available_gb = available as f64 / 1024.0 / 1024.0;

        let bar_len = 20;
        let filled_len = (percentage / 100.0 * bar_len as f64) as usize;
        let bar = "*".repeat(filled_len) + &"-".repeat(bar_len - filled_len);

        println!("\rRAM [{}] {:.1}% | {:.2} GB / {:.2} GB", bar, percentage, available_gb, total_gb);

        use std::io::Write;
        std::io::stdout().flush().unwrap();

        thread::sleep(Duration::from_secs(1));
    }
}
