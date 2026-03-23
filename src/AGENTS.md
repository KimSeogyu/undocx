<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-22 | Updated: 2026-03-22 -->

# src

## Purpose
Main Rust source code for the undocx converter. Implements a three-stage pipeline: DOCX parsing → AST extraction → Markdown rendering. The pipeline is pluggable via trait-based abstraction.

## Key Files

| File | Description |
|------|-------------|
| `lib.rs` | Library entry point — exports public API (`DocxToMarkdown`, `ConvertOptions`, `ImageHandling`), Python bindings behind `python` feature |
| `main.rs` | CLI entry point using `clap` — accepts input/output paths and image handling flags |
| `error.rs` | Error types via `thiserror`: `DocxParse`, `Io`, `Conversion`, `RelationshipNotFound`, `MissingReference`, `Zip`, `MediaNotFound` |
| `localization.rs` | Heading style parsing — maps DOCX style names ("Heading1", "Title") to heading levels |

## Subdirectories

| Directory | Purpose |
|-----------|---------|
| `adapters/` | Input format adapters — `AstExtractor` trait and DOCX implementation (see `adapters/AGENTS.md`) |
| `converter/` | Core conversion logic — paragraph, table, run, image, hyperlink, numbering, styles (see `converter/AGENTS.md`) |
| `core/` | AST definitions — `DocumentAst`, `BlockNode`, `ReferenceDefinitions` (see `core/AGENTS.md`) |
| `render/` | Output rendering — `Renderer` trait, Markdown renderer, escape utilities (see `render/AGENTS.md`) |

## For AI Agents

### Working In This Directory
- Public API changes must follow SemVer rules in `docs/API_POLICY.md`
- `AstExtractor` and `Renderer` traits are public integration points — signature changes require a major version bump
- Python bindings in `lib.rs` are gated by `#[cfg(feature = "python")]`
- The `ConversionContext` is the shared mutable state carrier — be careful with lifetime annotations

### Testing Requirements
- `cargo test` runs unit tests in each module
- `cargo test --all-features` includes Python binding compilation
- Integration and snapshot tests are in the top-level `tests/` directory

### Common Patterns
- Converters are stateless structs with `::convert()` associated functions taking `&ConversionContext`
- Errors propagate via `Result<T>` using the `?` operator
- `rs_docx` types use `Cow<'a, str>` for zero-copy parsing — respect lifetime constraints

## Dependencies

### Internal
- `deps/rs-docx` — low-level DOCX XML parsing

### External
- `clap` — CLI argument parsing (only in `main.rs`)
- `base64` — image encoding
- `zip` — DOCX archive extraction
- `thiserror` — error derive macros
- `pyo3` (optional) — Python bindings

<!-- MANUAL: -->
