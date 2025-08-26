# nextaudit (prototype)

This repository contains a prototype Rust-native CLI for auditing Next.js projects.

Run the binary with `cargo run -- scan` from the project root.

See the collected implementations and ecosystem notes in `docs/implementations.md`.

## Quickstart

1. Copy the example config into your project root:

```bash
cp .nextaudit.example.toml .nextaudit.toml
```

2. Run a scan (defaults to current directory and JSON output):

```bash
cargo run -- scan
```

3. Run tests:

```bash
cargo test
```

