/// The intermediate representation of a parsed DOCX document.
///
/// Produced by an [`AstExtractor`](crate::adapters::docx::AstExtractor) and consumed
/// by a [`Renderer`](crate::render::Renderer). This is the bridge between the
/// parsing and rendering stages of the conversion pipeline.
///
/// ```text
/// DOCX body ──▶ AstExtractor ──▶ DocumentAst ──▶ Renderer ──▶ Markdown
/// ```
#[derive(Debug, Clone, Default)]
pub struct DocumentAst {
    /// The document's content blocks, in source order.
    pub blocks: Vec<BlockNode>,
    /// Footnote, endnote, and comment definitions referenced by the content.
    pub references: ReferenceDefinitions,
}

/// A single block-level element in the document.
///
/// Each variant holds the already-rendered string content for that block.
/// The [`Renderer`](crate::render::Renderer) joins these blocks and appends
/// reference definitions.
#[derive(Debug, Clone)]
pub enum BlockNode {
    /// A Markdown paragraph (may include headings, lists, blockquotes, or
    /// inline formatting such as bold, italic, links, and images).
    Paragraph(String),
    /// An HTML `<table>` element with optional `colspan`/`rowspan` attributes.
    TableHtml(String),
    /// Raw HTML or fenced code blocks (```` ``` ````).
    RawHtml(String),
}

/// Collected reference definitions for footnotes, endnotes, and comments.
///
/// These are appended after the main content by the [`Renderer`](crate::render::Renderer)
/// using Markdown footnote syntax (`[^N]: text`).
#[derive(Debug, Clone, Default)]
pub struct ReferenceDefinitions {
    /// Footnote texts, indexed from 1 (`[^1]`, `[^2]`, ...).
    pub footnotes: Vec<String>,
    /// Endnote texts, indexed from 1 (`[^en1]`, `[^en2]`, ...).
    pub endnotes: Vec<String>,
    /// Comment `(id, text)` pairs, rendered as `[^cID]: text`.
    pub comments: Vec<(String, String)>,
}
