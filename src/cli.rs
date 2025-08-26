use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser, Debug)]
#[clap(name = "nextaudit", version)]
pub struct Opts {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Scan a project and emit a report
    Scan {
        /// Path to project (defaults to current dir)
        #[clap(short, long, default_value = ".")]
        path: String,
    },
    /// Write or show collected implementations/docs
    Docs {
        /// If set, write docs to disk (default: print path)
        #[clap(short, long)]
        write: bool,
    },
    /// Print configuration schema or init a config
    Config {
        #[clap(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    Init,
    Schema,
}

impl Opts {
    pub fn run(self) -> Result<()> {
        match self.command {
            Commands::Scan { path } => {
                println!("Scanning: {}", path);
                crate::engine::run_scan(&path)?;
            }
            Commands::Docs { write } => {
                if write {
                    crate::docs::write_docs()?;
                    println!("Wrote docs/implementations.md");
                } else {
                    println!("See docs/implementations.md in the project root.");
                }
            }
            Commands::Config { action } => match action {
                ConfigAction::Init => println!("Write a starter .nextaudit.toml"),
                ConfigAction::Schema => println!("Schema: (stub)"),
            },
        }
        Ok(())
    }
}
