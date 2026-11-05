use std::process::Command;
use std::fs::File;
use std::io::{BufRead, Write, Read};
use regex::Regex;

fn extract_host(host_str: &str) -> &str {
    if let Some(stripped) = host_str.strip_prefix("http://") {
        stripped
    } else if let Some(stripped) = host_str.strip_prefix("https://") {
        stripped
    } else {
        host_str
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <urls_file>", args[0]);
        eprintln!("Optional args: --output <file> --range <range> --timeout <ms> --tries <num> --batch-size <num> --delay <seconds>");
        std::process::exit(1);
    }

    let mut urls_file = "urls.txt";
    let mut output_file = "found_ports.txt".to_string();
    let mut port_range = "0-65535".to_string();
    let mut timeout = 1500;
    let mut tries = 1;
    let mut batch_size = 4500;
    let mut delay = 0.0;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--output" => {
                if i + 1 < args.len() {
                    output_file = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("Missing value for --output");
                    std::process::exit(1);
                }
            }
            "--range" => {
                if i + 1 < args.len() {
                    port_range = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("Missing value for --range");
                    std::process::exit(1);
                }
            }
            "--timeout" => {
                if i + 1 < args.len() {
                    timeout = args[i + 1].parse().expect("Timeout must be a number");
                    i += 2;
                } else {
                    eprintln!("Missing value for --timeout");
                    std::process::exit(1);
                }
            }
            "--tries" => {
                if i + 1 < args.len() {
                    tries = args[i + 1].parse().expect("Tries must be a number");
                    i += 2;
                } else {
                    eprintln!("Missing value for --tries");
                    std::process::exit(1);
                }
            }
            "--batch-size" => {
                if i + 1 < args.len() {
                    batch_size = args[i + 1].parse().expect("Batch size must be a number");
                    i += 2;
                } else {
                    eprintln!("Missing value for --batch-size");
                    std::process::exit(1);
                }
            }
            "--delay" => {
                if i + 1 < args.len() {
                    delay = args[i + 1].parse().expect("Delay must be a number");
                    i += 2;
                } else {
                    eprintln!("Missing value for --delay");
                    std::process::exit(1);
                }
            }
            _ => {
                if urls_file == "urls.txt" {
                    urls_file = &args[i];
                }
                i += 1;
            }
        }
    }

    let mut file = File::open(urls_file).expect("Unable to open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Unable to read file");

    let mut output = File::create(&output_file).expect("Unable to create output file");

    let open_regex = Regex::new(r"Open\s+([^:]+):(\d+)").unwrap();
    let greppable_regex = Regex::new(r"([^\s]+)\s*->\s*\[([^]]+)\]").unwrap();

    for line in content.lines() {
        let host = line.trim();
        if host.is_empty() || host.starts_with('#') {
            continue;
        }

        let target_host = extract_host(host);

        if target_host.is_empty() {
            eprintln!("Invalid target: {} (no hostname found)", host);
            continue;
        }

        let cmd = Command::new("rustscan")
            .arg("-a")
            .arg(&target_host)
            .arg("-r")
            .arg(&port_range)
            .arg("-g")
            .arg("-n")
            .arg("--timeout")
            .arg(timeout.to_string())
            .arg("--tries")
            .arg(tries.to_string())
            .arg("-b")
            .arg(batch_size.to_string())
            .output()
            .expect("Failed to execute rustscan");

        if cmd.status.success() {
            let stdout = String::from_utf8_lossy(&cmd.stdout);
            for line in stdout.lines() {
                if let Some(captures) = open_regex.captures(line) {
                    let host = captures.get(1).unwrap().as_str();
                    let port = captures.get(2).unwrap().as_str();
                    writeln!(output, "{}:{}", host, port).expect("Unable to write to file");
                } else if let Some(captures) = greppable_regex.captures(line) {
                    let host = captures.get(1).unwrap().as_str();
                    let ports_str = captures.get(2).unwrap().as_str();
                    for port_str in ports_str.split(',') {
                        if !port_str.trim().is_empty() {
                            writeln!(output, "{}:{}", host, port_str.trim()).expect("Unable to write to file");
                        }
                    }
                }
            }
        } else {
            eprintln!("Failed to scan {}", target_host);
        }

        if delay > 0.0 {
            std::thread::sleep(std::time::Duration::from_secs_f64(delay));
        }
    }

    println!("Scan completed. Open ports saved to {}", output_file);
}
