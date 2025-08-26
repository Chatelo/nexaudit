use crate::engine::Issue;
use anyhow::Result;
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
struct Report {
    issues: Vec<Issue>,
}

pub fn emit_report(issues: &[Issue]) -> Result<()> {
    let report = Report { issues: issues.to_vec() };
    let json = serde_json::to_string_pretty(&report)?;
    fs::write("nextaudit-report.json", json)?;
    println!("Wrote nextaudit-report.json ({} issues)", report.issues.len());
    Ok(())
}
