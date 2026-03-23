<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-22 | Updated: 2026-03-22 -->

# workflows

## Purpose
GitHub Actions workflow definitions for CI/CD automation.

## Key Files

| File | Description |
|------|-------------|
| `release.yml` | Release workflow — builds artifacts for multiple platforms, publishes to crates.io and PyPI. Triggered by version tags on main |
| `perf-benchmark.yml` | Performance benchmarking workflow — runs benchmarks and checks thresholds |
| `release-notes.yml` | Generates release notes from git history |

## For AI Agents

### Working In This Directory
- Release workflow requires tags to point to main branch history (enforced by policy)
- Performance benchmarks run on PRs to catch regressions early
- Workflows call scripts from `scripts/` — keep them in sync

### Testing Requirements
- Test workflow changes in a PR to verify they pass
- The release workflow builds for multiple targets — ensure cross-compilation compatibility

## Dependencies

### Internal
- `scripts/run_perf_benchmark.sh`, `scripts/generate_release_notes.sh`, `scripts/check_perf_threshold.sh`

### External
- GitHub Actions runners (ubuntu-latest, macos-latest, windows-latest)

<!-- MANUAL: -->
