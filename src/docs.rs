use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn write_docs() -> Result<()> {
    let src = Path::new("docs/implementations.md");
    let dst_dir = Path::new("docs");
    if !dst_dir.exists() {
        fs::create_dir_all(dst_dir)?;
    }
    // If the file already exists in docs/, overwrite it with the version in the repo (this is idempotent)
    let content = include_str!("../docs/implementations.md");
    fs::write(src, content)?;
    Ok(())
}
