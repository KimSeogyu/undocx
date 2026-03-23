<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-22 | Updated: 2026-03-22 -->

# docs

## Purpose
Project documentation and policy files.

## Key Files

| File | Description |
|------|-------------|
| `API_POLICY.md` | API stability policy — SemVer rules, deprecation process, quality gates, and dependency injection contract guarantees |

## Subdirectories

| Directory | Purpose |
|-----------|---------|
| `images/` | Documentation images and screenshots |

## For AI Agents

### Working In This Directory
- `API_POLICY.md` is the authoritative reference for what constitutes a breaking change
- Consult this policy before modifying any public trait or struct signature
- Quality gates defined here (`clippy`, tests, golden snapshots, benchmarks) must pass for releases

### Testing Requirements
- No automated tests — these are policy documents reviewed by humans

## Dependencies

### Internal
- Referenced by contributors and CI when evaluating API changes

<!-- MANUAL: -->
