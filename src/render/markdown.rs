use crate::core::ast::{BlockNode, DocumentAst};
use crate::render::Renderer;
use crate::Result;

/// The built-in Markdown renderer.
///
/// Joins [`BlockNode`]s with double newlines and appends
/// [reference definitions](crate::core::ast::ReferenceDefinitions) (footnotes,
/// endnotes, comments) after a `---` separator.
#[derive(Debug, Default, Clone, Copy)]
pub struct MarkdownRenderer;

impl Renderer for MarkdownRenderer {
    fn render(&self, document: &DocumentAst) -> Result<String> {
        let mut out = String::new();

        for block in &document.blocks {
            let rendered = match block {
                BlockNode::Paragraph(text)
                | BlockNode::TableHtml(text)
                | BlockNode::RawHtml(text) => text,
            };
            if rendered.is_empty() {
                continue;
            }
            out.push_str(rendered);
            out.push_str("\n\n");
        }

        let refs = &document.references;
        if !refs.footnotes.is_empty() || !refs.endnotes.is_empty() || !refs.comments.is_empty() {
            out.push_str("---\n\n");
            for (i, note) in refs.footnotes.iter().enumerate() {
                out.push_str(&format!("[^{}]: {}\n", i + 1, note));
            }
            for (i, note) in refs.endnotes.iter().enumerate() {
                out.push_str(&format!("[^en{}]: {}\n", i + 1, note));
            }
            for (id, text) in &refs.comments {
                out.push_str(&format!("[^c{}]: {}\n", id, text));
            }
        }

        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ast::{DocumentAst, ReferenceDefinitions};

    #[test]
    fn test_renderer_appends_references() {
        let doc = DocumentAst {
            blocks: vec![BlockNode::Paragraph("A".to_string())],
            references: ReferenceDefinitions {
                footnotes: vec!["note".to_string()],
                endnotes: Vec::new(),
                comments: Vec::new(),
            },
        };
        let rendered = MarkdownRenderer.render(&doc).expect("render should work");
        assert!(rendered.contains("A"));
        assert!(rendered.contains("[^1]: note"));
    }
}
