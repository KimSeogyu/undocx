//! Input format adapters for the conversion pipeline.
//!
//! This module defines the [`AstExtractor`] trait and provides the built-in
//! [`DocxExtractor`]. Implement `AstExtractor` to customize how DOCX body
//! content is transformed into a [`DocumentAst`](crate::core::ast::DocumentAst),
//! then pass it to
//! [`DocxToMarkdown::with_components`](crate::DocxToMarkdown::with_components).

mod extractor;

use crate::converter::ConversionContext;
use crate::core::ast::DocumentAst;
use crate::Result;
use rs_docx::document::BodyContent;

/// Extracts a [`DocumentAst`](crate::core::ast::DocumentAst) from parsed DOCX
/// body content.
///
/// The default implementation is [`DocxExtractor`], which delegates to
/// specialized converters for paragraphs, tables, and runs. To use a custom
/// extractor, pass it to
/// [`DocxToMarkdown::with_components`](crate::DocxToMarkdown::with_components).
///
/// # Implementing
///
/// ```no_run
/// use undocx::adapters::docx::AstExtractor;
/// use undocx::converter::ConversionContext;
/// use undocx::core::ast::{BlockNode, DocumentAst};
/// use undocx::Result;
/// use rs_docx::document::BodyContent;
///
/// struct MyExtractor;
///
/// impl AstExtractor for MyExtractor {
///     fn extract<'a>(
///         &self,
///         body: &[BodyContent<'a>],
///         context: &mut ConversionContext<'a>,
///     ) -> Result<DocumentAst> {
///         Ok(DocumentAst::default())
///     }
/// }
/// ```
///
/// # Errors
///
/// Returns [`Error`](crate::Error) if extraction fails (e.g., an I/O error
/// while reading an embedded image).
pub trait AstExtractor {
    /// Walk the DOCX body content and produce the intermediate AST.
    fn extract<'a>(
        &self,
        body: &[BodyContent<'a>],
        context: &mut ConversionContext<'a>,
    ) -> Result<DocumentAst>;
}

pub use extractor::DocxExtractor;
