<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-22 | Updated: 2026-03-22 -->

# core

## Purpose
Defines the internal Abstract Syntax Tree (AST) that serves as the intermediate representation between DOCX extraction and Markdown rendering.

## Key Files

| File | Description |
|------|-------------|
| `mod.rs` | Re-exports the `ast` submodule |
| `ast.rs` | AST types: `DocumentAst` (blocks + references), `BlockNode` (Paragraph, TableHtml, RawHtml), `ReferenceDefinitions` (footnotes, endnotes, comments) |

## For AI Agents

### Working In This Directory
- `BlockNode` is an enum — adding a new variant affects both extractors and renderers
- `DocumentAst` is the contract between `AstExtractor` and `Renderer` — changes here are breaking
- Keep types simple and serialization-friendly
- `ReferenceDefinitions` collects footnotes (indexed), endnotes (indexed), and comments (id + text pairs)

### Testing Requirements
- AST types are tested indirectly through converter and renderer tests
- `Default` is derived for `DocumentAst` and `ReferenceDefinitions`

## Dependencies

### Internal
None — this is a leaf module with no internal dependencies.

<!-- MANUAL: -->
