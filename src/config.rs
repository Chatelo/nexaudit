use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::Path;


#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub project: Option<Project>,
    pub ignore: Option<Vec<String>>, // e.g. [".git", "target", "node_modules"]
    pub thresholds: Option<Thresholds>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Thresholds {
    pub large_file_kb: Option<u32>, // e.g. 200
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub router: Option<String>,
    pub target: Option<String>,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let cfg_path = Path::new(path).join(".nextaudit.toml");
        if cfg_path.exists() {
            let contents = fs::read_to_string(&cfg_path)
                .with_context(|| format!("reading config {}", cfg_path.display()))?;
            let cfg: Config = toml::from_str(&contents)?;
            Ok(cfg)
        } else {
            Ok(Config::default())
        }
    }
}
