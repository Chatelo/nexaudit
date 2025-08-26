<!-- Implementations and examples gathered from: -->
<!-- - https://rust-cli.github.io/book/index.html -->
<!-- - https://www.rust-lang.org/what/cli -->
<!-- - https://blog.rust-lang.org/inside-rust/2025/07/21/sunsetting-the-rustwasm-github-org/ -->

# Rust CLI implementations and examples

The table below summarizes example implementations, patterns, and tooling referenced by the linked pages (Rust CLI Book, Rust website CLI guide, and Rust blog post).

| Source | Example / Pattern | Language / Crate | Purpose / Notes |
|---|---:|---|---|
| rust-cli book | Minimal clap-based CLI (Parser derive) | clap (derive) | Shows struct-based args parsing; tutorial starts a small tool that reads a file and prints N lines. This is the canonical beginner pattern (clap v3/v4 derive). |
| rust-cli book | Packaging & docs | cargo, clap_mangen / clap_mangen::Man | Recommends packaging a single static binary and generating man pages (clap_mangen helps produce man pages). |
| rust-cli book | Machine-friendly IO | serde_json, serde | Advice to offer both human and machine outputs (JSON/SARIF/etc); prefer serde for structured output. |
| rust-cli book | Logging & diagnostics | tracing, tracing-subscriber, log | Recommends structured logging; use `tracing` for spans and levels, `tracing-subscriber` for formatting and sinks. |
| rust-cli book | Concurrency & parallelism | rayon, tokio | For CPU-bound file analysis use `rayon` (data parallelism); for async IO tasks use `tokio`. |
| rust-lang.org (CLI page) | clap example snippet | clap | Short example showing how to define inputs and build a simple tool using derive macros. |
| rust-lang.org (CLI page) | Configuration handling | directories, toml, serde | Guidance to handle platform config paths and parse TOML/JSON via `serde` and `toml` crates. |
| rust-blog (rustwasm) | wasm-bindgen transfer note | wasm-bindgen, wasm-pack, walrus, gloo, twiggy | Operational note: the `rustwasm` org is being archived and `wasm-bindgen` transferred — if your CLI interacts with wasm tools be aware of repository moves and pin or vendor critical deps. |
| rust-blog (rustwasm) | Archival guidance | repository maintenance | Advises forking or pinning dependencies and identifying maintainers for critical wasm tooling. |

Additional recommended crates observed across the resources and relevant to this project:

- clap (v3/v4) — argument parsing with derive support
- serde / serde_json — structured input/output
- tracing / tracing-subscriber — structured logging and diagnostics
- walkdir — efficient recursive filesystem walking
- rayon — parallel file processing for CPU-bound rules
- tempfile or std::env/temp_dir — for test harnesses and temporary files

Quick implementation notes:
- Prefer producing both human-readable and machine-readable outputs (use feature flags or --format). The repo already emits JSON report files; extendable to SARIF/JSON-lines.
- Use `rayon` for rule execution across many files (already present in `src/rules.rs`).
- For packaging, build a single static binary and provide man pages via `clap_mangen` or similar.


Notes:
- The table condenses examples and guidance across the three pages. For implementation-ready code, the rust-cli book contains a step-by-step tutorial and sample code for a beginner-friendly CLI that can be extended into production tools.
- The rustwasm blog post does not include CLI patterns but is important when considering WebAssembly bindings and distribution choices.
