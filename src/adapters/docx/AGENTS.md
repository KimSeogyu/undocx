<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-22 | Updated: 2026-03-22 -->

# docx

## Purpose
DOCX-specific implementation of the `AstExtractor` trait. Walks the `rs_docx` body content tree and delegates to specialized converters (paragraph, table, run) to build the `DocumentAst`.

## Key Files

| File | Description |
|------|-------------|
| `mod.rs` | Defines the `AstExtractor` trait and re-exports `DocxExtractor` |
| `extractor.rs` | `DocxExtractor` — iterates body content, handles SDT, bookmarks, table cells, and delegates to `ParagraphConverter`, `TableConverter`, `RunConverter` |

## For AI Agents

### Working In This Directory
- `DocxExtractor` is the default extractor used by `DocxToMarkdown::new()`
- Each `BodyContent` variant is matched and dispatched to the appropriate converter
- SDT (Structured Document Tags) are recursively traversed
- Bookmarks emit `<a id="..."></a>` anchors using `escape_html_attr`

### Testing Requirements
- Tests in `converter/mod.rs` cover `convert_content` paths including SDT and bookmarks
- Custom extractor tests (`FakeExtractor`, `MissingRefExtractor`) validate the trait contract

### Common Patterns
- Pattern-match on `BodyContent` variants exhaustively
- Empty paragraphs are filtered out (no empty `BlockNode::Paragraph`)

## Dependencies

### Internal
- `converter::{ParagraphConverter, TableConverter, RunConverter, ConversionContext}`
- `core::ast::{BlockNode, DocumentAst}`
- `render::escape_html_attr`

### External
- `rs_docx::document` — parsed DOCX body content types

<!-- MANUAL: -->
