<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-22 | Updated: 2026-03-22 -->

# examples

## Purpose
Usage examples demonstrating how to use undocx from Rust and Python, plus a performance benchmark.

## Key Files

| File | Description |
|------|-------------|
| `perf_benchmark.rs` | Performance benchmarking — measures conversion speed across sample documents |

## Subdirectories

| Directory | Purpose |
|-----------|---------|
| `rust_example/` | Standalone Rust project showing library usage |
| `python_example/` | Python script demonstrating PyO3 bindings |

## For AI Agents

### Working In This Directory
- `rust_example/` is a separate Cargo project with its own `Cargo.toml`
- `python_example/` uses `uv` for dependency management and requires the `python` feature
- `perf_benchmark.rs` is used by CI via `scripts/run_perf_benchmark.sh`

### Testing Requirements
- Examples should compile and run successfully against the current library version
- Update examples when public API changes

<!-- MANUAL: -->
