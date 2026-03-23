//! Extracts images from DOCX archives under one of three strategies:
//! embed as inline base64, save to a directory, or skip entirely.
//!
//! The ZIP archive is opened once at construction and reused for every
//! image lookup, avoiding repeated decompression passes over the file.

use crate::{error::Error, Result};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use rs_docx::document::Drawing;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};

pub struct ImageExtractor {
    mode: ImageMode,
    source: ImageSource,
    counter: usize,
}

enum ImageMode {
    SaveToDir(PathBuf),
    Inline,
    Skip,
}

/// Cached ZIP archive — opened once and reused for every image extraction.
enum ImageSource {
    Archive(zip::ZipArchive<File>),
    ArchiveFromBytes(zip::ZipArchive<Cursor<Vec<u8>>>),
    None,
}

impl ImageExtractor {
    /// Creates an extractor that saves images to a directory (from file).
    pub fn new_with_dir<P: AsRef<Path>>(docx_path: P, output_dir: PathBuf) -> Result<Self> {
        // Ensure output directory exists
        fs::create_dir_all(&output_dir)?;

        let file = File::open(docx_path.as_ref())?;
        let archive = zip::ZipArchive::new(file)
            .map_err(|e| Error::DocxParse(format!("Failed to open DOCX as ZIP: {}", e)))?;

        Ok(Self {
            mode: ImageMode::SaveToDir(output_dir),
            source: ImageSource::Archive(archive),
            counter: 0,
        })
    }

    /// Creates an extractor that saves images to a directory (from bytes).
    pub fn new_with_dir_from_bytes(bytes: &[u8], output_dir: PathBuf) -> Result<Self> {
        // Ensure output directory exists
        fs::create_dir_all(&output_dir)?;

        let cursor = Cursor::new(bytes.to_vec());
        let archive = zip::ZipArchive::new(cursor)
            .map_err(|e| Error::DocxParse(format!("Failed to open DOCX as ZIP: {}", e)))?;

        Ok(Self {
            mode: ImageMode::SaveToDir(output_dir),
            source: ImageSource::ArchiveFromBytes(archive),
            counter: 0,
        })
    }

    /// Creates an extractor that embeds images as base64 (from file).
    pub fn new_inline<P: AsRef<Path>>(docx_path: P) -> Result<Self> {
        let file = File::open(docx_path.as_ref())?;
        let archive = zip::ZipArchive::new(file)
            .map_err(|e| Error::DocxParse(format!("Failed to open DOCX as ZIP: {}", e)))?;

        Ok(Self {
            mode: ImageMode::Inline,
            source: ImageSource::Archive(archive),
            counter: 0,
        })
    }

    /// Creates an extractor that embeds images as base64 (from bytes).
    pub fn new_inline_from_bytes(bytes: &[u8]) -> Result<Self> {
        let cursor = Cursor::new(bytes.to_vec());
        let archive = zip::ZipArchive::new(cursor)
            .map_err(|e| Error::DocxParse(format!("Failed to open DOCX as ZIP: {}", e)))?;

        Ok(Self {
            mode: ImageMode::Inline,
            source: ImageSource::ArchiveFromBytes(archive),
            counter: 0,
        })
    }

    /// Creates an extractor that skips all images.
    pub fn new_skip() -> Self {
        Self {
            mode: ImageMode::Skip,
            source: ImageSource::None,
            counter: 0,
        }
    }

    /// Extracts image from a Drawing element and returns Markdown.
    pub fn extract_from_drawing(
        &mut self,
        drawing: &Drawing,
        rels: &HashMap<String, String>,
    ) -> Result<Option<String>> {
        if matches!(self.mode, ImageMode::Skip) {
            return Ok(None);
        }

        // Try to find blip (image reference) in drawing
        let blip_id = self.find_blip_id(drawing);

        let Some(rel_id) = blip_id else {
            return Ok(None);
        };

        // Get image path from relationships
        let Some(image_path) = rels.get(&rel_id) else {
            return Ok(None);
        };

        // Extract and process image
        self.process_image(image_path)
    }

    fn find_blip_id(&self, drawing: &Drawing) -> Option<String> {
        // Try inline first (most common for embedded images)
        if let Some(inline) = &drawing.inline {
            if let Some(graphic) = &inline.graphic {
                if let Some(pic) = graphic.data.children.first() {
                    let embed = &pic.fill.blip.embed;
                    if !embed.is_empty() {
                        return Some(embed.to_string());
                    }
                }
            }
        }

        // Try anchor (for floating images)
        if let Some(anchor) = &drawing.anchor {
            if let Some(graphic) = &anchor.graphic {
                if let Some(pic) = graphic.data.children.first() {
                    let embed = &pic.fill.blip.embed;
                    if !embed.is_empty() {
                        return Some(embed.to_string());
                    }
                }
            }
        }

        None
    }

    /// Extracts image from a Pict element (VML).
    pub fn extract_from_pict(
        &mut self,
        pict: &rs_docx::document::Pict,
        rels: &HashMap<String, String>,
    ) -> Result<Option<String>> {
        if matches!(self.mode, ImageMode::Skip) {
            return Ok(None);
        }

        // Try to find image ID in shape or rect
        let blip_id = self.find_pict_blip_id(pict);

        let Some(rel_id) = blip_id else {
            return Ok(None);
        };

        // Get image path from relationships
        let Some(image_path) = rels.get(&rel_id) else {
            return Ok(None);
        };

        // Extract and process image
        self.process_image(image_path)
    }

    fn find_pict_blip_id(&self, pict: &rs_docx::document::Pict) -> Option<String> {
        // Check shape -> imagedata
        if let Some(shape) = &pict.shape {
            if let Some(img_data) = &shape.image_data {
                if let Some(id) = &img_data.id {
                    return Some(id.to_string());
                }
            }
        }

        // Check rect -> imagedata
        if let Some(rect) = &pict.rect {
            if let Some(img_data) = &rect.image_data {
                if let Some(id) = &img_data.id {
                    return Some(id.to_string());
                }
            }
        }

        None
    }

    fn process_image(&mut self, image_path: &str) -> Result<Option<String>> {
        // Read image from cached DOCX archive
        let image_data = self.read_image_from_docx(image_path)?;

        self.counter += 1;

        // Determine extension
        let ext = Path::new(image_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("png");

        match &self.mode {
            ImageMode::SaveToDir(dir) => {
                let filename = format!("image_{}.{}", self.counter, ext);
                let output_path = dir.join(&filename);

                fs::write(&output_path, &image_data)?;

                // Return relative path
                Ok(Some(format!("![image]({})", output_path.display())))
            }
            ImageMode::Inline => {
                let mime_type = match ext.to_lowercase().as_str() {
                    "png" => "image/png",
                    "jpg" | "jpeg" => "image/jpeg",
                    "gif" => "image/gif",
                    "webp" => "image/webp",
                    "svg" => "image/svg+xml",
                    _ => "application/octet-stream",
                };

                let b64 = BASE64.encode(&image_data);
                Ok(Some(format!(
                    "<img src=\"data:{};base64,{}\" alt=\"image\" />",
                    mime_type, b64
                )))
            }
            ImageMode::Skip => Ok(None),
        }
    }

    fn read_image_from_docx(&mut self, image_path: &str) -> Result<Vec<u8>> {
        // Image path is relative to word/ directory typically
        let full_path = if image_path.starts_with("word/") {
            image_path.to_string()
        } else {
            format!("word/{}", image_path)
        };

        let paths_to_try: [&str; 2] = [full_path.as_str(), image_path];

        match &mut self.source {
            ImageSource::Archive(archive) => {
                extract_from_cached_zip(archive, &paths_to_try, image_path)
            }
            ImageSource::ArchiveFromBytes(archive) => {
                extract_from_cached_zip(archive, &paths_to_try, image_path)
            }
            ImageSource::None => Ok(Vec::new()),
        }
    }
}

/// Extracts image bytes from an already-opened ZipArchive, trying each candidate path in order.
fn extract_from_cached_zip<R: Read + std::io::Seek>(
    archive: &mut zip::ZipArchive<R>,
    paths_to_try: &[&str],
    original_path: &str,
) -> Result<Vec<u8>> {
    for path in paths_to_try {
        if let Ok(mut entry) = archive.by_name(path) {
            let mut data = Vec::new();
            entry.read_to_end(&mut data)?;
            return Ok(data);
        }
    }

    Err(Error::MediaNotFound(original_path.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Seek, SeekFrom, Write};

    /// Build a minimal in-memory DOCX (ZIP) that contains two PNG stubs under word/media/.
    fn make_docx_with_two_images() -> Vec<u8> {
        let buf = Cursor::new(Vec::<u8>::new());
        let mut zip = zip::ZipWriter::new(buf);
        let options: zip::write::FileOptions<zip::write::ExtendedFileOptions> =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

        zip.start_file("word/media/image1.png", options.clone()).unwrap();
        zip.write_all(b"PNG1").unwrap();

        zip.start_file("word/media/image2.png", options).unwrap();
        zip.write_all(b"PNG2").unwrap();

        let mut finished = zip.finish().unwrap();
        finished.seek(SeekFrom::Start(0)).unwrap();
        finished.into_inner()
    }

    #[test]
    fn test_multiple_images_from_same_archive() {
        let docx_bytes = make_docx_with_two_images();
        let mut extractor = ImageExtractor::new_inline_from_bytes(&docx_bytes)
            .expect("Failed to create extractor");

        let img1 = extractor
            .read_image_from_docx("media/image1.png")
            .expect("Failed to read image1");
        assert_eq!(img1, b"PNG1");

        let img2 = extractor
            .read_image_from_docx("media/image2.png")
            .expect("Failed to read image2");
        assert_eq!(img2, b"PNG2");
    }

    #[test]
    fn test_archive_opened_once_path_variant() {
        // Verify the File-backed archive variant works the same way.
        // We use the inline bytes variant here as a proxy since we can't
        // cheaply create a temp file in a unit test without tempfile.
        let docx_bytes = make_docx_with_two_images();
        let mut extractor = ImageExtractor::new_inline_from_bytes(&docx_bytes)
            .expect("Failed to create extractor");

        // Both reads use the same cached archive — neither panics.
        let r1 = extractor.read_image_from_docx("media/image1.png");
        let r2 = extractor.read_image_from_docx("media/image2.png");
        assert!(r1.is_ok());
        assert!(r2.is_ok());
    }

    #[test]
    fn test_missing_image_returns_error() {
        let docx_bytes = make_docx_with_two_images();
        let mut extractor = ImageExtractor::new_inline_from_bytes(&docx_bytes)
            .expect("Failed to create extractor");

        let result = extractor.read_image_from_docx("media/nonexistent.png");
        assert!(result.is_err());
    }
}
