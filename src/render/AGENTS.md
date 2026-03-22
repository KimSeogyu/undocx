<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-22 | Updated: 2026-03-22 -->

# render

## Purpose
Output rendering layer. Defines the `Renderer` trait and provides the default `MarkdownRenderer` that serializes the AST to Markdown text. Includes escape utilities for safe Markdown and HTML output.

## Key Files

| File | Description |
|------|-------------|
| `mod.rs` | `Renderer` trait definition: `fn render(&self, document: &DocumentAst) -> Result<String>` |
| `markdown.rs` | `MarkdownRenderer` — renders `DocumentAst` blocks separated by double newlines, appends footnote/endnote/comment reference definitions after a `---` separator |
| `escape.rs` | Escape utilities: `escape_html_attr` (HTML attribute encoding), `escape_markdown_link_text` (bracket escaping), `escape_markdown_link_destination` (parenthesis escaping) |

## For AI Agents

### Working In This Directory
- `Renderer` is a public trait — signature changes are breaking API changes
- The rendering is intentionally simple: blocks are joined with `\n\n`, references appended at the end
- To add a new output format, implement `Renderer` and use `DocxToMarkdown::with_components()`
- Escape functions must handle all OWASP-relevant characters for their context

### Testing Requirements
- Unit tests in `markdown.rs` cover reference rendering (footnotes, endnotes, comments)
- Escape functions should be tested with edge cases (empty strings, special characters)

### Common Patterns
- Each `BlockNode` variant is rendered as its inner string content
- Empty blocks are skipped
- Reference definitions use Markdown footnote syntax: `[^N]: text`

## Dependencies

### Internal
- `core::ast` — `DocumentAst`, `BlockNode`, `ReferenceDefinitions`

<!-- MANUAL: -->
