<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-22 | Updated: 2026-03-22 -->

# adapters

## Purpose
Input format adapters that convert parsed document structures into the internal AST. Defines the `AstExtractor` trait — the public integration point for custom extraction pipelines.

## Key Files

| File | Description |
|------|-------------|
| `mod.rs` | Re-exports the `docx` submodule |

## Subdirectories

| Directory | Purpose |
|-----------|---------|
| `docx/` | DOCX-specific extractor implementation (see `docx/AGENTS.md`) |

## For AI Agents

### Working In This Directory
- `AstExtractor` is a public trait — any signature change is a breaking API change
- To add a new input format, create a new submodule implementing `AstExtractor`
- The trait takes `&[BodyContent]` and `&mut ConversionContext` and returns `Result<DocumentAst>`

### Testing Requirements
- Unit tests for extractors live alongside their implementation
- Integration tests in `tests/integration_test.rs` exercise the full pipeline

## Dependencies

### Internal
- `converter::ConversionContext` — shared conversion state
- `core::ast::DocumentAst` — output AST type

<!-- MANUAL: -->
