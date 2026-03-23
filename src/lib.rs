//! # undocx
//!
//! DOCX to Markdown converter using `rs_docx`.
//!
//! ## Example
//!
//! ```no_run
//! use undocx::{DocxToMarkdown, ConvertOptions, ImageHandling};
//!
//! let options = ConvertOptions {
//!     image_handling: ImageHandling::SaveToDir("./images".into()),
//!     ..Default::default()
//! };
//!
//! let converter = DocxToMarkdown::new(options);
//! let markdown = converter.convert("document.docx").unwrap();
//! println!("{}", markdown);
//! ```
//!
//! ## Advanced Example (Custom Extractor/Renderer)
//!
//! ```no_run
//! use undocx::adapters::docx::AstExtractor;
//! use undocx::converter::ConversionContext;
//! use undocx::core::ast::{BlockNode, DocumentAst};
//! use undocx::render::Renderer;
//! use undocx::{ConvertOptions, DocxToMarkdown, Result};
//! use rs_docx::document::BodyContent;
//!
//! #[derive(Debug, Default, Clone, Copy)]
//! struct MyExtractor;
//!
//! impl AstExtractor for MyExtractor {
//!     fn extract<'a>(
//!         &self,
//!         _body: &[BodyContent<'a>],
//!         _context: &mut ConversionContext<'a>,
//!     ) -> Result<DocumentAst> {
//!         Ok(DocumentAst {
//!             blocks: vec![BlockNode::Paragraph("custom pipeline".to_string())],
//!             references: Default::default(),
//!         })
//!     }
//! }
//!
//! #[derive(Debug, Default, Clone, Copy)]
//! struct MyRenderer;
//!
//! impl Renderer for MyRenderer {
//!     fn render(&self, document: &DocumentAst) -> Result<String> {
//!         Ok(format!("blocks={}", document.blocks.len()))
//!     }
//! }
//!
//! fn main() -> Result<()> {
//!     let converter = DocxToMarkdown::with_components(
//!         ConvertOptions::default(),
//!         MyExtractor,
//!         MyRenderer,
//!     );
//!     let output = converter.convert("document.docx")?;
//!     println!("{}", output);
//!     Ok(())
//! }
//! ```

pub mod adapters;
pub mod converter;
pub mod core;
pub mod error;
pub mod localization;
pub mod render;

pub use converter::DocxToMarkdown;
pub use error::{Error, Result};
pub use localization::parse_heading_style;

use std::path::PathBuf;

/// Options for DOCX to Markdown conversion.
#[derive(Debug, Clone)]
pub struct ConvertOptions {
    /// How to handle images in the document.
    pub image_handling: ImageHandling,
    /// Whether to preserve exact whitespace.
    pub preserve_whitespace: bool,
    /// Whether to use HTML for underlined text.
    pub html_underline: bool,
    /// Whether to use HTML for strikethrough text.
    pub html_strikethrough: bool,
    /// Whether to fail conversion when a referenced note/comment cannot be resolved.
    pub strict_reference_validation: bool,
}

impl Default for ConvertOptions {
    fn default() -> Self {
        Self {
            image_handling: ImageHandling::Inline,
            preserve_whitespace: false,
            html_underline: true,
            html_strikethrough: false,
            strict_reference_validation: false,
        }
    }
}

/// Specifies how images should be handled during conversion.
#[derive(Debug, Clone)]
pub enum ImageHandling {
    /// Save images to a directory and reference them by path.
    SaveToDir(PathBuf),
    /// Embed images as base64 data URIs.
    Inline,
    /// Skip images entirely.
    Skip,
}

// Python bindings (only when 'python' feature is enabled)
#[cfg(feature = "python")]
mod python_bindings {
    use super::*;
    use pyo3::prelude::*;
    use pyo3::types::PyBytes;

    /// Converts a DOCX file to Markdown.
    ///
    /// Argument can be a file path (str) or file content (bytes).
    ///
    /// Optional keyword arguments:
    /// - image_handling: "inline" | "skip" | path string (save images to that directory)
    /// - preserve_whitespace: bool
    /// - html_underline: bool
    /// - html_strikethrough: bool
    /// - strict_reference_validation: bool
    #[pyfunction]
    #[pyo3(signature = (input, *, image_handling=None, preserve_whitespace=None, html_underline=None, html_strikethrough=None, strict_reference_validation=None))]
    fn convert_docx(
        input: &Bound<'_, PyAny>,
        image_handling: Option<String>,
        preserve_whitespace: Option<bool>,
        html_underline: Option<bool>,
        html_strikethrough: Option<bool>,
        strict_reference_validation: Option<bool>,
    ) -> PyResult<String> {
        let mut options = ConvertOptions::default();

        if let Some(handling) = image_handling {
            options.image_handling = match handling.as_str() {
                "inline" => ImageHandling::Inline,
                "skip" => ImageHandling::Skip,
                path => ImageHandling::SaveToDir(PathBuf::from(path)),
            };
        }
        if let Some(v) = preserve_whitespace {
            options.preserve_whitespace = v;
        }
        if let Some(v) = html_underline {
            options.html_underline = v;
        }
        if let Some(v) = html_strikethrough {
            options.html_strikethrough = v;
        }
        if let Some(v) = strict_reference_validation {
            options.strict_reference_validation = v;
        }

        let converter = DocxToMarkdown::new(options);

        if let Ok(path) = input.extract::<String>() {
            converter
                .convert(&path)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
        } else if let Ok(bytes) = input.downcast::<PyBytes>() {
            converter
                .convert_from_bytes(bytes.as_bytes())
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Expected string path or bytes",
            ))
        }
    }

    /// A Python module implemented in Rust.
    #[pymodule]
    pub fn undocx(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(convert_docx, m)?)?;
        Ok(())
    }
}
