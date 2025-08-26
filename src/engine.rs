use anyhow::Result;
use tracing::info;

pub fn run_scan(path: &str) -> Result<()> {
    info!(%path, "starting scan");

    // High-level orchestration: parse config, build graph, run rules, produce report
    let _config = crate::config::Config::load(path)?;
    let issues = crate::rules::run_all(path)?;
    crate::reporter::emit_report(&issues)?;
    Ok(())
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Issue {
    pub id: String,
    pub severity: String,
    pub message: String,
}

// (rule execution moved to src/rules.rs)

