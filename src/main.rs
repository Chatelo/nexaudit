mod cli;
mod config;
mod engine;
mod reporter;
mod rules;

use tracing_subscriber::fmt::format::FmtSpan;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let opts = cli::Opts::parse();
    opts.run()?;
    Ok(())
}
