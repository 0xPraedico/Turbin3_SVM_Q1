use serde_json::Value;
use std::fs;
use std::path::Path;
use prettytable::{Table, Row, Cell};

fn main() {
    let base_path = Path::new("target/criterion");
    let mut table = Table::new();

    // Add table header
    table.add_row(Row::new(vec![
        Cell::new("Benchmark"),
        Cell::new("Average Time (ns)"),
        Cell::new("Standard Deviation"),
    ]));

    // Process Single Transaction results
    process_group(&mut table, base_path, "Single Transaction Comparison");
    
    // Process Bulk Transaction results
    process_group(&mut table, base_path, "Bulk Transactions (100) Comparison");

    // Print the table
    table.printstd();
}

fn process_group(table: &mut Table, base_path: &Path, group_name: &str) {
    let group_path = base_path.join(group_name);
    
    if let Ok(entries) = fs::read_dir(group_path) {
        for entry in entries.flatten() {
            let path = entry.path().join("new/estimates.json");
            if path.exists() {
                if let Ok(content) = fs::read_to_string(path) {
                    if let Ok(json) = serde_json::from_str::<Value>(&content) {
                        let mean = json["mean"]["point_estimate"].as_f64().unwrap_or(0.0);
                        let std_dev = json["std_dev"]["point_estimate"].as_f64().unwrap_or(0.0);
                        
                        table.add_row(Row::new(vec![
                            Cell::new(&entry.file_name().to_string_lossy()),
                            Cell::new(&format!("{:.2}", mean)),
                            Cell::new(&format!("{:.2}", std_dev)),
                        ]));
                    }
                }
            }
        }
    }
} 