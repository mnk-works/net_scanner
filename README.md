# Net Scanner Tool

A Rust CLI tool for scanning multiple targets for open ports and capturing results.

## Description

This tool allows you to scan multiple targets for open ports using rustscan. It reads a subscription file, parses each target (including http:// and https:// URLs), scans each target using rustscan, and saves the results to an output file.

## Installation

1. Clone this repository:
```bash
ggit clone <repository-url>
cd net_scanner
cargo build --release
```

2. Ensure rustscan is installed on your system:
```bash
# On Ubuntu/Debian / Kali / Parrot
cargo install rustscan

# Or using package manager (if available):
# sudo apt install rustscan          # Debian/Ubuntu/Kali
# brew install rustscan              # macOS
# pkg install rustscan              # Android/termux
# yay rustscan                     # Arch Linux
# snap install rustscan             # Snap-based systems

# Or download from https://github.com/RustScan/RustScan
```

## Usage

```bash
./target/release/net_scanner <urls_file> [options]
```

### Required Arguments

- `<urls_file>`: Path to a text file containing one target per line (e.g., google.com, https://github.com)

### Optional Arguments

- `--output <file>`: Output file to save open ports (default: found_ports.txt)
- `--range <range>`: Port range to scan (default: "0-65535")
- `--timeout <ms>`: Timeout in milliseconds (default: 1500)
- `--tries <num>`: Number of connection attempts (default: 1)
- `--batch-size <num>`: Batch size for scanning (default: 4500)
- `--delay <seconds>`: Delay between targets in seconds (default: 0)

### Examples

```bash
# Basic usage with default settings
./target/release/net_scanner urls.txt

# Scan with custom output file and port range
./target/release/net_scanner urls.txt --output my_ports.txt --range "0-1000"

# Scan with custom timeout and delay
./target/release/net_scanner urls.txt --timeout 500 --delay 1
```

## Input File Format

The input file should contain one target per line. Targets can be:

- Domain names (e.g., google.com)
- IP addresses (e.g., 142.251.209.238)
- With http:// or https:// prefixes (e.g., http://google.com, https://github.com)
- Comment lines (starting with "#") are ignored
- Empty lines are ignored

## Output Format

The output file contains one line per open port in the format:

```
host:port
```

For example:
```
142.251.209.238:80
142.251.209.238:443
127.0.0.1:3389
```

## Features

- **URL Parsing**: Automatically extracts hostnames from URLs with http:// or https:// prefixes
- **Robust Scanning**: Uses rustscan's "greppable" output format for reliable parsing
- **Flexible Configuration**: Supports custom port ranges, timeouts, delays, and batch sizes
- **Append Mode**: Appends to existing output files, preserving previous scans
- **Error Handling**: Gracefully handles invalid targets and scan failures

## Notes

- rustscan must be in your PATH for the tool to function
- Multiple scans of the same host will append additional entries
- The tool supports targets in various formats for maximum flexibility