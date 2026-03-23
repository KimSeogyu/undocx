//! # undocx
//!
//! Fast DOCX to Markdown converter.
//!
//! ## Quick Start
//!
//! ```no_run
//! let md = undocx::convert("document.docx").unwrap();
//! ```
//!
//! ## With Options
//!
//! ```no_run
//! let md = undocx::builder()
//!     .skip_images()
//!     .strict()
//!     .convert("document.docx")
//!     .unwrap();
//! ```
//!
//! ## Reusable Converter
//!
//! ```no_run
//! let converter = undocx::Converter::builder()
//!     .save_images_to("./images")
//!     .build();
//!
//! let md = converter.convert("document.docx").unwrap();
//! ```
//!
//! ## Custom Pipeline
//!
//! See [`DocxToMarkdown::with_components`] for custom extractor/renderer pipelines.

pub mod adapters;
pub mod converter;
pub mod core;
pub mod error;
pub mod localization;
pub mod render;

pub use converter::DocxToMarkdown;
pub use error::{Error, Result};
pub use localization::parse_heading_style;

/// Type alias for the default converter configuration.
///
/// For custom extractors or renderers, use [`DocxToMarkdown::with_components`].
pub type Converter = DocxToMarkdown;

use std::path::{Path, PathBuf};

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

// ── Builder ────────────────────────────────────────────────────────

/// Fluent builder for configuring DOCX to Markdown conversion.
///
/// Created via [`builder()`] or [`Converter::builder()`].
///
/// # Example
///
/// ```no_run
/// // Build and convert in one step
/// let md = undocx::builder()
///     .skip_images()
///     .preserve_whitespace()
///     .convert("report.docx")
///     .unwrap();
///
/// // Or build a reusable converter
/// let converter = undocx::builder()
///     .save_images_to("./images")
///     .build();
///
/// let md = converter.convert("a.docx").unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct Builder {
    options: ConvertOptions,
}

impl Builder {
    /// Creates a new builder with default options.
    pub fn new() -> Self {
        Self {
            options: ConvertOptions::default(),
        }
    }

    // ── Image Handling ──────────────────────────────

    /// Skip images entirely from the output.
    pub fn skip_images(mut self) -> Self {
        self.options.image_handling = ImageHandling::Skip;
        self
    }

    /// Embed images as inline base64 data URIs (this is the default).
    pub fn inline_images(mut self) -> Self {
        self.options.image_handling = ImageHandling::Inline;
        self
    }

    /// Save images to the given directory and reference them by path.
    pub fn save_images_to(mut self, dir: impl Into<PathBuf>) -> Self {
        self.options.image_handling = ImageHandling::SaveToDir(dir.into());
        self
    }

    // ── Formatting ──────────────────────────────────

    /// Preserve exact whitespace from the document.
    pub fn preserve_whitespace(mut self) -> Self {
        self.options.preserve_whitespace = true;
        self
    }

    /// Control HTML `<u>` output for underlined text (default: `true`).
    pub fn html_underline(mut self, enabled: bool) -> Self {
        self.options.html_underline = enabled;
        self
    }

    /// Control HTML `<s>` output for strikethrough text (default: `false`).
    pub fn html_strikethrough(mut self, enabled: bool) -> Self {
        self.options.html_strikethrough = enabled;
        self
    }

    /// Fail when a referenced note or comment cannot be resolved.
    pub fn strict(mut self) -> Self {
        self.options.strict_reference_validation = true;
        self
    }

    // ── Terminal Operations ─────────────────────────

    /// Build a reusable [`Converter`].
    pub fn build(self) -> Converter {
        Converter::new(self.options)
    }

    /// Build and immediately convert a DOCX file at the given path.
    pub fn convert(self, path: impl AsRef<Path>) -> Result<String> {
        self.build().convert(path)
    }

    /// Build and immediately convert DOCX bytes.
    pub fn convert_bytes(self, bytes: &[u8]) -> Result<String> {
        self.build().convert_bytes(bytes)
    }

    /// Build and immediately convert DOCX from a reader.
    pub fn convert_reader(self, reader: impl std::io::Read + std::io::Seek) -> Result<String> {
        self.build().convert_reader(reader)
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

// ── Convenience Functions ──────────────────────────────────────────

/// Convert a DOCX file at the given path to Markdown.
///
/// This is the simplest way to use undocx. For configuration, use [`builder()`].
///
/// # Example
///
/// ```no_run
/// let md = undocx::convert("report.docx").unwrap();
/// ```
pub fn convert(path: impl AsRef<Path>) -> Result<String> {
    Converter::with_defaults().convert(path)
}

/// Convert DOCX bytes to Markdown.
///
/// # Example
///
/// ```no_run
/// let bytes = std::fs::read("report.docx").unwrap();
/// let md = undocx::convert_bytes(&bytes).unwrap();
/// ```
pub fn convert_bytes(bytes: &[u8]) -> Result<String> {
    Converter::with_defaults().convert_bytes(bytes)
}

/// Convert DOCX from a reader to Markdown.
///
/// The reader must implement both [`Read`](std::io::Read) and
/// [`Seek`](std::io::Seek) because DOCX is a ZIP archive.
///
/// # Example
///
/// ```no_run
/// let file = std::fs::File::open("report.docx").unwrap();
/// let md = undocx::convert_reader(file).unwrap();
/// ```
pub fn convert_reader(reader: impl std::io::Read + std::io::Seek) -> Result<String> {
    Converter::with_defaults().convert_reader(reader)
}

/// Start building a configured converter.
///
/// Shorthand for [`Builder::new()`].
///
/// # Example
///
/// ```no_run
/// let md = undocx::builder()
///     .skip_images()
///     .convert("report.docx")
///     .unwrap();
/// ```
pub fn builder() -> Builder {
    Builder::new()
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
                .convert_bytes(bytes.as_bytes())
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
