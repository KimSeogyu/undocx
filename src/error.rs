//! Conversion error types for undocx.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse DOCX file: {0}")]
    DocxParse(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Conversion error: {0}")]
    Conversion(String),

    #[error("Relationship not found: {0}")]
    RelationshipNotFound(String),

    /// Returned only when `strict_reference_validation` is enabled and a
    /// footnote, endnote, or comment reference cannot be resolved against
    /// the source document.
    #[error("Missing reference: {0}")]
    MissingReference(String),

    #[error("ZIP error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("Media not found: {0}")]
    MediaNotFound(String),
}
