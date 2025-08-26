use nextaudit::config::{Config, Thresholds};
use nextaudit::rules::run_all;
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
