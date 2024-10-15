#![windows_subsystem = "windows"]

use chrono::Local;
use scraper::{Html, Selector};
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    if !cfg!(target_os = "windows") {
        println!("This program only works available on Windows");
        return;
    }

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let new_path: PathBuf = current_dir.join("battery-reports");

    create_battery_report(&new_path);

    let battery_report_path = new_path.join("battery-report.html");

    let file_content =
        fs::read_to_string(&battery_report_path).expect("Failed to read battery report.");

    // println!("HTML Content:\n{}", file_content);

    let document = Html::parse_document(&file_content);

    let design_capacity_selector = Selector::parse(
        "body > table:nth-child(5) > tbody:nth-child(3) > tr:nth-child(5) > td:nth-child(2)",
    )
    .expect("Invalid selector");

    for (i, element) in document.select(&design_capacity_selector).enumerate() {
        println!("Element {}: {:?}", i, element.text().collect::<Vec<_>>());
    }

    let design_capacity_element = document
        .select(&design_capacity_selector)
        .nth(0)
        .expect("Failed to find the first element matching the selector design_capacity_selector");

    let full_charge_capacity_selector = Selector::parse(
        "body > table:nth-child(5) > tbody:nth-child(3) > tr:nth-child(7) > td:nth-child(2)",
    )
    .expect("Invalid selector");
    let full_charge_capacity_element = document
        .select(&full_charge_capacity_selector)
        .nth(0)
        .expect(
            "Failed to find the first element matching the selector full_charge_capacity_selector",
        );

    let design_capacity_text = design_capacity_element.text().collect::<Vec<_>>().join("");
    let full_charge_capacity_text = full_charge_capacity_element
        .text()
        .collect::<Vec<_>>()
        .join("");

    let design_capacity = design_capacity_text
        .replace('\u{202f}', "")
        .trim_end_matches(" mWh\n\n      ")
        .to_string();
    let full_charge_capacity = full_charge_capacity_text
        .replace('\u{202f}', "")
        .trim_end_matches(" mWh\n\n      ")
        .to_string();

    println!("Design capacity: {} mWh", design_capacity);
    println!("Full charge capacity: {} mWh", full_charge_capacity);

    save_to_csv(&design_capacity, &full_charge_capacity);
}

fn save_to_csv(design_capacity: &str, full_charge_capacity: &str) {
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let file_path = "battery_capacity.csv";

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .expect("Failed to open file");

    if file.metadata().unwrap().len() == 0 {
        // Write the header if the file is empty
        writeln!(file, "timestamp,design_capacity,full_charge_capacity")
            .expect("Failed to write header");
    }

    writeln!(
        file,
        "{},{},{}",
        timestamp, design_capacity, full_charge_capacity
    )
    .expect("Failed to write record");
}

fn create_battery_report(directory: &PathBuf) {
    if !(directory.exists()) {
        fs::create_dir_all(&directory).expect("Failed to create directory");
    }

    let output = Command::new("cmd")
        .current_dir(&directory)
        .args([
            "/C",
            "powercfg",
            "/batteryreport",
            "/output",
            "battery-report.html",
        ])
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("Command failed with status: {}", output.status);
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    } else {
        println!("Command executed successfully.");
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    }
}
