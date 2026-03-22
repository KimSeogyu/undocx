<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-22 | Updated: 2026-03-22 -->

# scripts

## Purpose
CI/CD and build infrastructure scripts for benchmarking, release notes generation, and performance threshold checking.

## Key Files

| File | Description |
|------|-------------|
| `run_perf_benchmark.sh` | Runs performance benchmarks against sample documents, outputs results to `output_tests/perf/latest.json` |
| `generate_release_notes.sh` | Generates release notes from git history |
| `check_perf_threshold.sh` | Validates benchmark results against performance thresholds — used as a CI gate |

## For AI Agents

### Working In This Directory
- Scripts are called from GitHub Actions workflows in `.github/workflows/`
- `run_perf_benchmark.sh` depends on `examples/perf_benchmark.rs`
- Performance thresholds prevent regressions — update thresholds only when intentional

### Testing Requirements
- Run scripts locally before modifying to understand current behavior
- Ensure scripts remain compatible with both local development and CI environments

<!-- MANUAL: -->
