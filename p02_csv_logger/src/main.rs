use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;
use chrono::Local;

fn main() {
    let sys_path = "/proc/loadavg";
    let csv_output_path = "/root/workspace/cpu_history.csv";

    println!("Writing data to {}", csv_output_path);
    println!("CTRL + C for stop");

    if !std::path::Path::new(csv_output_path).exists() {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(csv_output_path)
            .expect("ERROR: Can not create the file");

        writeln!(file, "timestamp,load").expect("ERROR: Can not write the headers");
    }

    loop {
        // Extract content
        let content = std::fs::read_to_string(sys_path)
            .expect("ERROR: Can not read /proc/loadavg");
        let str_load = content.split_whitespace().next().unwrap();
        
        // Transform: ISO 8601
        let date = Local::now();
        let date_fmt = date.format("%Y-%m-%d %H:%M:%S");

        // Load: CSV append
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(csv_output_path)
            .expect("ERROR: Can not open the CSV file");

        if let Err(e) = writeln!(file, "{},{}", date_fmt, str_load) {
            eprintln!("ERROR: While writing at line: {}", e);
        } else {
            println!("Record save: {} | {}", date_fmt, str_load);
        }

        thread::sleep(Duration::from_secs(5));
    }
}
