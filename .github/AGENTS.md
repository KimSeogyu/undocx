<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-22 | Updated: 2026-03-22 -->

# .github

## Purpose
GitHub configuration — CI/CD workflows and code ownership.

## Key Files

| File | Description |
|------|-------------|
| `CODEOWNERS` | Code ownership rules for pull request reviews |

## Subdirectories

| Directory | Purpose |
|-----------|---------|
| `workflows/` | GitHub Actions workflow definitions (see `workflows/AGENTS.md`) |

## For AI Agents

### Working In This Directory
- Workflow changes affect CI/CD — test locally with `act` or verify in a PR before merging

### Testing Requirements
- Open a PR to verify workflow changes pass before merging

## Dependencies

### Internal
- Workflows call scripts from `scripts/`

<!-- MANUAL: -->
