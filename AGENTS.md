<!-- Generated: 2026-03-22 | Updated: 2026-03-22 -->

# undocx

## Purpose
A high-performance DOCX to Markdown converter written in Rust, with Python bindings via PyO3/Maturin. Provides both a CLI tool and a library with a pluggable architecture (custom extractors and renderers).

## Key Files

| File | Description |
|------|-------------|
| `Cargo.toml` | Rust project manifest — defines features (`python`), dependencies, and crate types (`cdylib`, `rlib`) |
| `pyproject.toml` | Python package config using Maturin build backend |
| `build.rs` | Rust build script |
| `README.md` | Project documentation |
| `test_samples.sh` | Script to run sample document conversions |

## Subdirectories

| Directory | Purpose |
|-----------|---------|
| `src/` | Main Rust source code — converter pipeline, AST, adapters, rendering (see `src/AGENTS.md`) |
| `deps/` | Vendored dependencies — `rs-docx` DOCX parser (see `deps/AGENTS.md`) |
| `tests/` | Integration, golden snapshot, and property-based tests (see `tests/AGENTS.md`) |
| `examples/` | Usage examples in Rust and Python (see `examples/AGENTS.md`) |
| `scripts/` | CI/CD and benchmark scripts (see `scripts/AGENTS.md`) |
| `docs/` | Project documentation and policies (see `docs/AGENTS.md`) |
| `samples/` | Real-world Korean financial/legal DOCX test documents |
| `.github/` | GitHub Actions workflows and CODEOWNERS (see `.github/AGENTS.md`) |

## For AI Agents

### Working In This Directory
- Rust edition 2021, MSRV 1.75
- Run `cargo test --all-features` to validate changes
- Run `cargo clippy --all-features --tests -- -D warnings` before committing
- Python bindings are behind the `python` feature flag
- Follow SemVer rules documented in `docs/API_POLICY.md`

### Testing Requirements
- Golden snapshot tests: `tests/golden_snapshot_test.rs`
- Integration tests: `tests/integration_test.rs`
- Property-based tests: `tests/invariant_randomized_test.rs`
- Benchmark: `scripts/run_perf_benchmark.sh`

### Architecture Overview
```
DOCX file → rs-docx parser → AstExtractor → DocumentAst → Renderer → Markdown
```
The pipeline is pluggable via `DocxToMarkdown::with_components(options, extractor, renderer)`.

### Common Patterns
- Trait-based abstraction: `AstExtractor` for input, `Renderer` for output
- `ConversionContext` carries mutable state through the conversion pipeline
- Error handling via `thiserror` with domain-specific variants

## Dependencies

### External
- `rs-docx` 0.2.0 — DOCX XML parsing (vendored in `deps/`)
- `clap` 4.5 — CLI argument parsing
- `base64` 0.22 — Image encoding
- `zip` 2.2 — DOCX archive extraction
- `thiserror` 2.0 — Error derive macros
- `pyo3` 0.23 (optional) — Python bindings

<!-- MANUAL: -->
