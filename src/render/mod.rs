//! Output rendering for the conversion pipeline.
//!
//! This module defines the [`Renderer`] trait and provides the built-in
//! [`MarkdownRenderer`]. Implement `Renderer` to produce custom output formats
//! (HTML, plain text, or a custom Markdown dialect) and pass it to
//! [`DocxToMarkdown::with_components`](crate::DocxToMarkdown::with_components).

mod escape;
mod markdown;

use crate::core::ast::DocumentAst;
use crate::Result;

pub use escape::{escape_html_attr, escape_markdown_link_destination, escape_markdown_link_text};
pub use markdown::MarkdownRenderer;

/// Serializes a [`DocumentAst`] into a final output string.
///
/// The default implementation is [`MarkdownRenderer`]. To use a custom renderer,
/// pass it to [`DocxToMarkdown::with_components`](crate::DocxToMarkdown::with_components).
///
/// # Implementing
///
/// ```no_run
/// use undocx::core::ast::DocumentAst;
/// use undocx::render::Renderer;
/// use undocx::Result;
///
/// struct PlainTextRenderer;
///
/// impl Renderer for PlainTextRenderer {
///     fn render(&self, document: &DocumentAst) -> Result<String> {
///         // Custom rendering logic here
///         Ok(String::new())
///     }
/// }
/// ```
///
/// # Errors
///
/// Returns [`Error`](crate::Error) if rendering fails.
pub trait Renderer {
    /// Render the document AST into a string.
    fn render(&self, document: &DocumentAst) -> Result<String>;
}
