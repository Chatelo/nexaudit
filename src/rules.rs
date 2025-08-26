#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, Thresholds};
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_html_missing_viewport() {
        let tmp = tempdir().unwrap();
        let file = tmp.path().join("test.html");
        fs::write(&file, "<html><head></head><body></body></html>").unwrap();
        let config = Config::default();
        let issues = run_all(tmp.path().to_str().unwrap(), &config).unwrap();
        assert!(issues.iter().any(|i| i.id == "a11y::viewport_missing"));
    }

    #[test]
    fn test_img_missing_alt() {
        let tmp = tempdir().unwrap();
        let file = tmp.path().join("test.html");
        fs::write(&file, "<img src='foo.png'>").unwrap();
        let config = Config::default();
        let issues = run_all(tmp.path().to_str().unwrap(), &config).unwrap();
        assert!(issues.iter().any(|i| i.id == "a11y::img_missing_alt"));
    }

    #[test]
    fn test_large_js_file() {
        let tmp = tempdir().unwrap();
        let file = tmp.path().join("big.js");
        let big_content = "a".repeat(250 * 1024); // 250 KB
        fs::write(&file, big_content).unwrap();
        let mut config = Config::default();
        config.thresholds = Some(Thresholds { large_file_kb: Some(200) });
        let issues = run_all(tmp.path().to_str().unwrap(), &config).unwrap();
        assert!(issues.iter().any(|i| i.id == "perf::large_file"));
    }

    #[test]
    fn test_ignore_dir() {
        let tmp = tempdir().unwrap();
        let ignore_dir = tmp.path().join("node_modules");
        fs::create_dir_all(&ignore_dir).unwrap();
        let file = ignore_dir.join("foo.js");
        fs::write(&file, "dangerouslySetInnerHTML").unwrap();
        let mut config = Config::default();
        config.ignore = Some(vec!["node_modules".to_string()]);
        let issues = run_all(tmp.path().to_str().unwrap(), &config).unwrap();
        assert!(!issues.iter().any(|i| i.id == "sec::dangerous_html"));
    }
}
use crate::engine::Issue;
use crate::config::Config;
use anyhow::Result;
use rayon::prelude::*;
use std::fs;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

fn is_ignored(entry: &DirEntry, ignore: &[String]) -> bool {
    if entry.depth() == 0 {
        return false;
    }
    if let Some(name) = entry.file_name().to_str() {
        // config-driven ignore
        if ignore.contains(&name.to_string()) {
            return true;
        }
        // ignore hidden directories (e.g., .cache)
        if name.starts_with('.') && entry.file_type().is_dir() {
            return true;
        }
    }
    false
}

pub fn run_all(path: &str, config: &Config) -> Result<Vec<Issue>> {
    let mut issues = Vec::new();

    // Use config ignore or default
    let ignore = config.ignore.as_ref().cloned().unwrap_or_else(|| vec![".git".into(), "target".into(), "node_modules".into(), "dist".into(), "build".into()]);
    let large_file_kb = config.thresholds.as_ref().and_then(|t| t.large_file_kb).unwrap_or(200);

    // collect file paths with filter_entry to skip ignored dirs
    let mut files = Vec::new();
    for entry in WalkDir::new(path).into_iter().filter_entry(|e| !is_ignored(e, &ignore)).filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            files.push(entry.path().to_owned());
        }
    }

    // run rules across files in parallel
    let file_issues: Vec<Vec<Issue>> = files
        .par_iter()
        .map(|p| run_rules_for_file(p, large_file_kb))
        .filter_map(|r| r.ok())
        .collect();

    for mut v in file_issues {
        issues.append(&mut v);
    }

    Ok(issues)
}

fn run_rules_for_file(path: &Path, large_file_kb: u32) -> Result<Vec<Issue>> {
    let mut issues = Vec::new();

    // Quick size guard: skip very large files (configurable)
    if let Ok(meta) = fs::metadata(path) {
        if meta.len() > (large_file_kb as u64) * 1024 * 10 {
            return Ok(issues);
        }
    }

    // Read bytes first so we can detect binary files
    let bytes = fs::read(path)?;
    // if contains NUL byte, treat as binary and skip
    if bytes.iter().any(|b| *b == 0) {
        return Ok(issues);
    }

    let content = String::from_utf8_lossy(&bytes);
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    // Example rule: HTML files should have a <meta name="viewport"> tag
    if ext.eq_ignore_ascii_case("html") || ext.eq_ignore_ascii_case("htm") {
        if !content.contains("<meta name=\"viewport\"") {
            issues.push(Issue {
                id: "a11y::viewport_missing".into(),
                severity: "warning".into(),
                message: format!("{}: missing <meta name=\"viewport\">", path.display()),
            });
        }
    }

    // Example heuristic: large JS/TS files
    if ext.eq_ignore_ascii_case("js") || ext.eq_ignore_ascii_case("ts") || ext.eq_ignore_ascii_case("tsx") || ext.eq_ignore_ascii_case("jsx") {
        let kb = content.len() as f64 / 1024.0;
        if kb > large_file_kb as f64 {
            issues.push(Issue {
                id: "perf::large_file".into(),
                severity: "warning".into(),
                message: format!("{}: large file (~{:.0} KB)", path.display(), kb),
            });
        }
    }

    // Security heuristic: usage of dangerouslySetInnerHTML in React code
    if content.contains("dangerouslySetInnerHTML") {
        issues.push(Issue {
            id: "sec::dangerous_html".into(),
            severity: "high".into(),
            message: format!("{}: usage of dangerouslySetInnerHTML (possible XSS)", path.display()),
        });
    }

    // Accessibility heuristic: <img> without alt attribute (basic check)
    if ext.eq_ignore_ascii_case("html") || ext.eq_ignore_ascii_case("htm") || ext.eq_ignore_ascii_case("jsx") || ext.eq_ignore_ascii_case("tsx") {
        for tag in content.match_indices("<img") {
            // take a slice from the tag start to the next '>' to inspect attributes
            if let Some(rest) = content.get(tag.0..) {
                if let Some(end) = rest.find('>') {
                    let tag_text = &rest[..end];
                    if !tag_text.contains("alt=") {
                        issues.push(Issue {
                            id: "a11y::img_missing_alt".into(),
                            severity: "warning".into(),
                            message: format!("{}: <img> tag without alt attribute", path.display()),
                        });
                    }
                }
            }
        }
    }

    Ok(issues)
}
