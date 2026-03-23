<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-22 | Updated: 2026-03-23 -->

# converter

## Purpose
Core conversion logic that orchestrates DOCX to Markdown transformation. Contains the main `DocxToMarkdown` struct and specialized converters for each document element type (paragraphs, tables, runs, images, hyperlinks, numbering, styles).

## Key Files

| File | Description |
|------|-------------|
| `mod.rs` | `DocxToMarkdown` — main orchestrator: parses DOCX, builds context, delegates to extractor/renderer. Supports `new()`, `builder()`, `with_components()`, `convert()`, `convert_bytes()`, `convert_reader()` |
| `context.rs` | `ConversionContext` — shared mutable state: relationship map, numbering, images, footnotes/endnotes/comments tracking, missing reference detection |
| `paragraph.rs` | `ParagraphConverter` — converts paragraphs to Markdown (headings, lists, blockquotes, regular text). Largest file (~43KB) |
| `run.rs` | `RunConverter` — converts text runs with formatting (bold, italic, underline, strikethrough, code) |
| `table.rs` | `TableConverter` — converts tables to HTML `<table>` elements |
| `table_grid.rs` | Table grid analysis — column width and span calculations |
| `numbering.rs` | `NumberingResolver` — resolves DOCX numbering definitions to ordered/unordered list markers |
| `styles.rs` | `StyleResolver` — resolves DOCX style IDs to style properties (heading levels, list styles) |
| `image.rs` | `ImageExtractor` — extracts images from DOCX ZIP archive (cached), supports save-to-dir, inline base64, and skip modes |

## For AI Agents

### Working In This Directory
- `paragraph.rs` is the most complex file — handles heading detection, list nesting, blockquotes, and inline formatting composition
- `ConversionContext` uses lifetime `'a` tied to the DOCX document — all borrows must respect this
- Converters are stateless — they take `&ConversionContext` and return `Result<String>`
- `DocxToMarkdown` is generic over `E: AstExtractor` and `R: Renderer` with defaults

### Testing Requirements
- Unit tests are in `mod.rs` (converter orchestration) and inline in each submodule
- Golden snapshot tests validate end-to-end output in `tests/golden_snapshot_test.rs`
- Test custom pipelines using `DocxToMarkdown::with_components()`

### Common Patterns
- `ParagraphConverter::convert()` and `TableConverter::convert()` are the main entry points
- Footnotes, endnotes, and comments are registered during extraction and rendered as Markdown reference-style links
- Missing references are tracked and reported when `strict_reference_validation` is enabled

## Dependencies

### Internal
- `core::ast` — `DocumentAst`, `BlockNode`, `ReferenceDefinitions`
- `adapters::docx` — `AstExtractor`, `DocxExtractor`
- `render` — `Renderer`, `MarkdownRenderer`, escape utilities

### External
- `rs_docx` — parsed DOCX types (`Docx`, `BodyContent`, `Paragraph`, `Table`, etc.)
- `base64` — image encoding (in `image.rs`)
- `zip` — DOCX archive reading (in `image.rs`)

<!-- MANUAL: -->
