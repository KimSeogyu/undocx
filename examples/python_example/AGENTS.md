<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-23 | Updated: 2026-03-23 -->

# python_example

## Purpose
Demonstrates the undocx Python bindings (`pyo3`/`maturin`). Shows file-path and byte-stream conversion with keyword options.

## Key Files

| File | Description |
|------|-------------|
| `demo.py` | End-to-end usage: path-based and bytes-based conversion with `image_handling` option |
| `pyproject.toml` | Python project config using maturin build backend |
| `uv.lock` | Locked dependency versions |

## Subdirectories

| Directory | Purpose |
|-----------|---------|
| (virtualenv directories created at runtime) | |

## For AI Agents

### Working In This Directory
- Build the Python wheel first: `maturin develop --features python` from the repo root
- Run with `uv run demo.py` or `python demo.py`
- The `undocx` import comes from the native extension, not a pure-Python package

### Testing Requirements
- Ensure a sample `.docx` file is available before running
- Verify output matches expected Markdown format

## Dependencies

### Internal
- `undocx` Python extension (built via `maturin develop --features python`)

### External
- `maturin` — build backend for PyO3 extensions

<!-- MANUAL: -->
