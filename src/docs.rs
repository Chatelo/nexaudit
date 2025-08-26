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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn test_write_docs_creates_file() -> Result<()> {
        let tmp = tempfile::tempdir()?;
        let old = env::current_dir()?;
        env::set_current_dir(tmp.path())?;

        // Ensure docs dir does not exist initially
        assert!(!Path::new("docs").exists());

        write_docs()?;

        // After write_docs, the file should exist
        let p = Path::new("docs/implementations.md");
        assert!(p.exists());
        let content = fs::read_to_string(p)?;
        assert!(content.contains("Rust CLI implementations and examples"));

        // restore cwd
        env::set_current_dir(old)?;
        Ok(())
    }
}
