use core::f64;
use std::process::{Command, Stdio, exit};
use std::time::{Duration, Instant};
use std::env;

const DEFAULT_RUNS: u16 = 16;

fn main() {
    let runs = env::var("RUNS")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(DEFAULT_RUNS);

    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: tim <command> [args...]");
        exit(1);
    }

    let mut durations = Vec::new();
    for _ in 0..runs {
        let start = Instant::now();
        let status = Command::new(&args[0])
            .args(&args[1..])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("Failed to execute command");
        let duration = start.elapsed();

        if !status.success() {
            eprintln!("Command failed with status: {}", status);
            exit(1);
        }

        durations.push(duration);
    }

    let cmd = args.join(" ");

    print_stats(&cmd, &durations, runs);
}

fn print_stats(cmd: &str, durations: &[Duration], runs: u16) {
    let durations_ms: Vec<f64> = durations
        .iter()
        .map(|d| d.as_secs_f64() * 1000.)
        .collect();

    let sum: f64 = durations_ms.iter().sum();
    let mean = sum / durations_ms.len() as f64;

    let variance = durations_ms
        .iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f64>()
        / durations_ms.len() as f64;

    let sd = variance.sqrt();
    let min = durations_ms.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = durations_ms.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    println!("Stats for:\n`{}`", cmd);
    println!("--------------------------------");
    println!("Runs: {}", runs);
    println!("Min:  {:.4} ms", min);
    println!("Mean: {:.4} ms", mean);
    println!("Max:  {:.4} ms", max);
    println!("SD:   {:.4} ms", sd);
}
