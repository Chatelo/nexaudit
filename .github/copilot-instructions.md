# Next.js Audit CLI – Unified Feature-Rich Build Prompt

## Executive Summary
Design and ship a **production-grade, Rust-native CLI tool** for comprehensive auditing of Next.js applications. The tool must deliver exhaustive static and dynamic analysis, security, performance, memory, accessibility, and best-practice checks. It should be distributed as a cross-platform binary and as Node/Bun/WASM packages, with zero-JS runtime logic and first-class CI/CD, IDE, and autofix integration.

## Core Architecture & Distribution
- **Language:** 100% Rust core, leveraging crates like `clap`, `tokio`, `serde`, `rayon`, `swc`, `oxc`, etc.
- **Bindings:** Node-API via `napi-rs` for Node/Bun; WASM build via `wasm-bindgen` for Deno/edge.
- **Packaging:**
  - npm: `@nextaudit/cli` (downloads correct binary), `@nextaudit/core` (WASM), plugins (JS wrappers)
  - Cargo: `cargo install nextaudit` for Rust users
  - Prebuilt binaries for Linux, macOS, Windows (x64/arm64)
- **Config:** `.nextaudit.toml` or `.nextauditrc.json/ts` with schema, extends, per-rule severity, ignore patterns
- **Performance:**
  - Cold scan < 30s on 50k LOC; re-scan < 5s with cache
  - Memory ceiling < 1.5 GB on large repos
  - Parallel file processing (rayon), incremental analysis, content-hash caching
- **Security:**
  - No network calls by default; all online checks require `--online`
  - Sandboxed file reads; never exec untrusted code

## CLI Surface & UX
- **Subcommands:** `scan`, `audit`, `fix`, `report`, `watch`, `config`, `route-map`, `bundle-analyze`, `rules`, `init`, `ci`, `format`
- **Options:**
  - `--format` (text, json, sarif, junit, md, csv, github, gitlab)
  - `--categories` (lint, deadcode, perf, sec, mem, bestpractices, a11y, seo, deps)
  - `--fix`, `--fix-strict` (safe/strict autofix)
  - `--online` (enable network checks)
  - `--baseline`, `--max-warnings`, `--fail-on`, `--concurrency`, `--cache-dir`, `--no-cache`, `--include`, `--exclude`, `--ignore-path`, `--min-severity`, `--max-issues`, `--profile`, `--output`, `--only`, `--except`, `--verbose`, `--quiet`, `--version`, `--help`
- **Output:** Human-readable (color, emoji), SARIF, JSON, JUnit, Markdown, CSV, diff-based, actionable recommendations
- **Integration:**
  - GitHub Actions, GitLab CI, Jenkins, Vercel/Netlify hooks
  - Pre-commit hooks, IDE extensions (VS Code, JetBrains), LSP diagnostics
  - Prometheus/DataDog metrics, webhook/Slack/Discord notifications

## Audit Surface Map
### Static Analysis
- **Linting:**
  - ESLint/Prettier rules re-implemented in Rust (Next.js, React, import/export, code style, complexity)
  - TypeScript strictness, unused types, config overrides
  - Import/export analysis, circular dependencies, unused imports/exports
  - React hooks correctness, RSC boundaries, Next.js conventions
  - Security lint rules (XSS, dangerouslySetInnerHTML, raw HTML, CSP, headers)
  - Accessibility (ARIA, semantic HTML, keyboard, color contrast, forms)
  - SEO (meta tags, Open Graph, structured data, sitemap, robots.txt, internal linking)
- **Dead Code Detection:**
  - Unused components, hooks, utilities, pages, API routes, assets, exports
  - Tree-shaking analysis, orphaned files, dynamic imports, unused CSS/classes
- **Performance:**
  - Bundle size per route/component, chunk analysis, heavy dependencies, code splitting, hydration, SSR/SSG
  - Core Web Vitals (LCP, FID, CLS), image/font optimization, data fetching, route-level splitting
- **Security:**
  - Dependency CVEs (offline/online), outdated packages, license compliance
  - Secret detection (.env, code, comments), SSRF, XSS, CSRF, SQL injection, unsafe imports
  - Next.js config, headers, API route security, authentication, client-side risks
- **Memory Leaks:**
  - Uncleaned event listeners, uncleared intervals/timeouts, improper useEffect cleanup, circular references
  - Resource management, unclosed connections, unsubscribed observables, DOM leaks
- **Best Practices:**
  - Routing, data fetching, config, middleware, environment usage, build optimization
  - Type safety, logging, i18n, testing posture
- **Dependency Management:**
  - Outdated/duplicate dependencies, bundle impact, tree-shaking, peer dependencies, license scanning

### Runtime Analysis (Optional)
- **Memory profiling:** Headless Chrome via Rust CDP, heap snapshots, event listener leaks
- **Performance traces:** TTFB, FCP, LCP, TTI, CLS, budgets
- **Security runtime:** CSP violations, reflected XSS, hydration mismatches
- **Bundle runtime:** Webpack stats, cross-reference static estimates

## Autofixers
- Safe autofixers for Next.js/React idioms, cleanup patterns, code style, security headers, etc.
- Strict autofixers for deeper refactors (guarded)
- Dry-run diff preview, idempotent diffs

## Configuration Example
```toml
[extends]
paths = ["@nextaudit/recommended"]

[rules]
next.image.required_dimensions = "error"
react.hooks.exhaustive_deps = "error"
perf.route_chunk_budget = { level = "warning", threshold_kb = 200 }
sec.csp.required = "warning"

[project]
router = "app" # or "pages" or "mixed"
target = "node" # or "edge"

[ci]
baseline = "./.nextaudit-baseline.json"
fail_on = "error"
```

## Architecture
```
nextaudit/
├── crates/
│   ├── nextaudit-core          # Rule engine, config, reporter
│   ├── nextaudit-parser        # SWC/TS AST + module graph
│   ├── nextaudit-linter        # All lint rules
│   ├── nextaudit-deadcode      # DCE & dep analysis
│   ├── nextaudit-security      # CVE, secrets, CSP
│   ├── nextaudit-perf          # Bundle cost, runtime perf
│   ├── nextaudit-memory        # Leak detection
│   ├── nextaudit-devtools      # Chrome CDP wrapper
│   ├── nextaudit-cli           # clap CLI, watch, cache
│   └── nextaudit-node          # N-API bindings
├── packages/
│   ├── @nextaudit/cli          # npm wrapper
│   ├── @nextaudit/core-wasm    # WASM build for web
│   └── vscode-nextaudit        # Extension
├── schema/
│   └── nextauditrc.schema.json
```
- Caching: content-based hashing (xxhash) of source + rules + config
- Parallelism: rayon + async-std for I/O, rule-level parallelism
- Incremental: only re-analyze changed files and their importers
- Language server: tower-lsp + tokio; diagnostics & quick-fixes

## Extensibility
- Rule SDK: Rust trait `Rule` + JS plugin bridge (WASM)
- Custom reporters: implement `Reporter` trait
- Custom parsers: WASM plugin returning custom AST nodes
- Preset packs: `@nextaudit/preset-enterprise`, `@nextaudit/preset-strict`

## Testing & Release
- Unit: rule fixtures & snapshots
- Integration: run on top 100 Next.js OSS repos nightly
- Regression: synthetic repo with known issues
- Performance: lint 1M LOC in <5s on M1
- Security: fuzz config parser, fuzz rule inputs
- CI/CD: GitHub Actions matrix, prebuild & upload `napi` artifacts, SBOM, provenance
- Semantic versioning, update notifications, migration guides

## Observability & Telemetry
- Opt-in anonymous metrics: rule hit rates, duration buckets; strictly no file paths or code
- Local `--profile` to emit timing spans per phase

## Security Hardening
- No dynamic require or eval
- Secrets scanner with entropy + pattern libraries; allowlist & git history optional scan
- Online checks fenced behind `--online` with clear disclosures
- Regular dependency review of Rust crates; supply chain pinned via `cargo deny` + `cargo audit`

## Developer Experience

## Changelog Discipline
- **Always update `CHANGELOG.md`**: Every code, config, or documentation change must be reflected in `CHANGELOG.md` with a clear, dated entry describing the change, rationale, and impact. This ensures traceability and transparency for all contributors and users.

## Roadmap & Stretch Goals
 - Chrome CDP micro-runner for live perf probes

## Roadmap & Stretch Goals
- Chrome CDP micro-runner for live perf probes
- SBOM generation, license audit, secrets scanning integration
- AI auto-fixes (optional WASM plugin)
- Multi-framework support (SvelteKit, Nuxt, Remix)
- VS Code/JetBrains extensions via WASM core

---
**Prompt Objective:**
Implement the above spec end-to-end. Prioritize core correctness, performance, and rule coverage. Maintain ruthless focus on zero-JS core, clean APIs, and enterprise-ready outputs. Ensure all features, integrations, and extensibility points are covered. The final artifact must rival and exceed existing solutions like ESLint, Lighthouse, and security scanners, specifically optimized for Next.js applications with the performance benefits of Rust.
