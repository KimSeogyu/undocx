<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-23 | Updated: 2026-03-23 -->

# rust_example

## Purpose
Standalone Rust binary demonstrating the undocx library API. Shows `ConvertOptions` configuration and file-based conversion.

## Key Files

| File | Description |
|------|-------------|
| `Cargo.toml` | Depends on `undocx` via path reference |
| `src/main.rs` | CLI that converts a DOCX file using `DocxToMarkdown::new()` |

## Subdirectories

| Directory | Purpose |
|-----------|---------|
| `src/` | Rust source code |

## For AI Agents

### Working In This Directory
- Build with `cargo build` from this directory (separate workspace)
- Prefer using the new convenience API: `undocx::convert()` or `undocx::builder()`
- Keep this example simple — it serves as a quick-start reference

### Testing Requirements
- Must compile against the current library version
- Update when public API changes

## Dependencies

### Internal
- `undocx` crate (path dependency)

<!-- MANUAL: -->
