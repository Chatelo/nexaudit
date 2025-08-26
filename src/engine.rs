use anyhow::Result;
use tracing::info;

pub fn run_scan(path: &str, output: &str, format: &str) -> Result<()> {
    info!(%path, %output, %format, "starting scan");

    // High-level orchestration: parse config, build graph, run rules, produce report
    let config = crate::config::Config::load(path)?;
    let issues = crate::rules::run_all(path, &config)?;
    crate::reporter::emit_report(&issues, output, format)?;
    Ok(())
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Issue {
    pub id: String,
    pub severity: String,
    pub message: String,
}

// (rule execution moved to src/rules.rs)

