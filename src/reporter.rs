use crate::engine::Issue;
use anyhow::Result;
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
struct Report {
    issues: Vec<Issue>,
}

pub fn emit_report(issues: &[Issue], out: &str, format: &str) -> Result<()> {
    let report = Report { issues: issues.to_vec() };
    match format.to_lowercase().as_str() {
        "json" => {
            let json = serde_json::to_string_pretty(&report)?;
            fs::write(out, json)?;
            println!("Wrote {} ({} issues)", out, report.issues.len());
        }
        "text" => {
            let mut s = String::new();
            for it in &report.issues {
                s.push_str(&format!("[{}] {} - {}\n", it.severity, it.id, it.message));
            }
            fs::write(out, s)?;
            println!("Wrote {} ({} issues)", out, report.issues.len());
        }
        other => {
            eprintln!("Unknown format '{}', defaulting to json", other);
            let json = serde_json::to_string_pretty(&report)?;
            fs::write(out, json)?;
            println!("Wrote {} ({} issues)", out, report.issues.len());
        }
    }
    Ok(())
}
