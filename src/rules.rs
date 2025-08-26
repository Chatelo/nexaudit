use crate::engine::Issue;
use anyhow::Result;
use rayon::prelude::*;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn run_all(path: &str) -> Result<Vec<Issue>> {
    let mut issues = Vec::new();

    // collect file paths (simple recursive walk)
    let mut files = Vec::new();
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            files.push(entry.path().to_owned());
        }
    }

    // run rules across files in parallel
    let file_issues: Vec<Vec<Issue>> = files
        .par_iter()
        .map(|p| run_rules_for_file(p))
        .filter_map(|r| r.ok())
        .collect();

    for mut v in file_issues {
        issues.append(&mut v);
    }

    Ok(issues)
}

fn run_rules_for_file(path: &Path) -> Result<Vec<Issue>> {
    let mut issues = Vec::new();
    let content = fs::read_to_string(path)?;
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
        if kb > 200.0 {
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
