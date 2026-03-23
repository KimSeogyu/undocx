# undocx API Guide

Comprehensive guide to the undocx DOCX-to-Markdown converter API.

## Installation

### Rust

```toml
[dependencies]
undocx = "0.5"
```

### Python

```bash
pip install undocx
```

### CLI

```bash
cargo install undocx
```

---

## Rust API

### Quick Conversion

The simplest way to convert a DOCX file — one function call:

```rust
use undocx;

fn main() -> undocx::Result<()> {
    let md = undocx::convert("report.docx")?;
    println!("{}", md);
    Ok(())
}
```

### Convert from Bytes

Useful when reading from S3, HTTP responses, or databases:

```rust
use undocx;

fn convert_from_s3(bytes: &[u8]) -> undocx::Result<String> {
    undocx::convert_bytes(bytes)
}
```

### Convert from Reader

Any type implementing `Read + Seek` works — including `File`, `Cursor`, and buffered readers:

```rust
use undocx;
use std::fs::File;

fn main() -> undocx::Result<()> {
    let file = File::open("report.docx").unwrap();
    let md = undocx::convert_reader(file)?;
    println!("{}", md);
    Ok(())
}
```

### Builder Pattern — Image Handling

Control how images are processed during conversion:

```rust
use undocx;

fn main() -> undocx::Result<()> {
    // Skip images entirely — optimal for RAG/LLM text extraction
    let md = undocx::builder()
        .skip_images()
        .convert("report.docx")?;

    // Or save images to a directory, with Markdown references
    let md = undocx::builder()
        .save_images_to("./images")
        .convert("report.docx")?;

    // Or embed as inline base64 (this is the default)
    let md = undocx::builder()
        .inline_images()
        .convert("report.docx")?;

    Ok(())
}
```

### Builder Pattern — Formatting and Validation

Combine multiple options for fine-grained control:

```rust
use undocx;

fn main() -> undocx::Result<()> {
    let md = undocx::builder()
        .skip_images()                 // omit images
        .preserve_whitespace()         // keep original spacing
        .html_underline(false)         // disable <u> tags
        .html_strikethrough(true)      // use <s> instead of ~~
        .strict()                      // fail on broken note/comment refs
        .convert("report.docx")?;
    println!("{}", md);
    Ok(())
}
```

### Reusable Converter

Build once, convert many files — avoids re-parsing options each time:

```rust
use undocx::Converter;

fn main() -> undocx::Result<()> {
    let converter = Converter::builder()
        .skip_images()
        .strict()
        .build();

    // Reuse for multiple files
    let md1 = converter.convert("quarterly-report.docx")?;
    let md2 = converter.convert("annual-summary.docx")?;

    // Also works with bytes
    let bytes = std::fs::read("contract.docx")?;
    let md3 = converter.convert_bytes(&bytes)?;

    Ok(())
}
```

### Custom Pipeline — Custom Renderer

Implement `Renderer` to produce any output format (plain text, HTML, JSON):

```rust
use undocx::core::ast::{BlockNode, DocumentAst};
use undocx::render::Renderer;
use undocx::{ConvertOptions, DocxToMarkdown, Result};

struct PlainTextRenderer;

impl Renderer for PlainTextRenderer {
    fn render(&self, document: &DocumentAst) -> Result<String> {
        let mut output = String::new();
        for block in &document.blocks {
            match block {
                BlockNode::Paragraph(text) => output.push_str(text),
                BlockNode::TableHtml(html) => output.push_str(html),
                BlockNode::RawHtml(html) => output.push_str(html),
            }
            output.push('\n');
        }
        Ok(output)
    }
}

fn main() -> Result<()> {
    use undocx::adapters::docx::DocxExtractor;

    let converter = DocxToMarkdown::with_components(
        ConvertOptions::default(),
        DocxExtractor,        // built-in extractor
        PlainTextRenderer,    // your custom renderer
    );
    let output = converter.convert("report.docx")?;
    println!("{}", output);
    Ok(())
}
```

### Custom Pipeline — Custom Extractor

Implement `AstExtractor` to customize how DOCX elements map to AST nodes:

```rust
use undocx::adapters::docx::AstExtractor;
use undocx::converter::ConversionContext;
use undocx::core::ast::{BlockNode, DocumentAst};
use undocx::render::MarkdownRenderer;
use undocx::{ConvertOptions, DocxToMarkdown, Result};
use rs_docx::document::BodyContent;

struct FilteredExtractor;

impl AstExtractor for FilteredExtractor {
    fn extract<'a>(
        &self,
        body: &[BodyContent<'a>],
        _context: &mut ConversionContext<'a>,
    ) -> Result<DocumentAst> {
        // Example: only extract paragraphs, skip tables
        let blocks: Vec<BlockNode> = body.iter().filter_map(|content| {
            if let BodyContent::Paragraph(p) = content {
                let text = p.text().to_string();
                if !text.is_empty() {
                    return Some(BlockNode::Paragraph(text));
                }
            }
            None
        }).collect();
        Ok(DocumentAst { blocks, references: Default::default() })
    }
}

fn main() -> Result<()> {
    let converter = DocxToMarkdown::with_components(
        ConvertOptions::default(),
        FilteredExtractor,
        MarkdownRenderer,
    );
    let output = converter.convert("report.docx")?;
    println!("{}", output);
    Ok(())
}
```

---

## Configuration Reference

### ImageHandling Enum

| Variant | Builder Method | Description |
|---------|---------------|-------------|
| `ImageHandling::Inline` | `.inline_images()` | Embed as base64 data URIs (default) |
| `ImageHandling::SaveToDir(path)` | `.save_images_to(dir)` | Save to directory, reference by path |
| `ImageHandling::Skip` | `.skip_images()` | Omit images entirely |

### ConvertOptions Fields

| Field | Type | Default | Builder Method |
|-------|------|---------|---------------|
| `image_handling` | `ImageHandling` | `Inline` | `.skip_images()` / `.inline_images()` / `.save_images_to()` |
| `preserve_whitespace` | `bool` | `false` | `.preserve_whitespace()` |
| `html_underline` | `bool` | `true` | `.html_underline(bool)` |
| `html_strikethrough` | `bool` | `false` | `.html_strikethrough(bool)` |
| `strict_reference_validation` | `bool` | `false` | `.strict()` |

### Builder Terminal Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `.build()` | `Converter` | Create a reusable converter |
| `.convert(path)` | `Result<String>` | Build and convert a file |
| `.convert_bytes(bytes)` | `Result<String>` | Build and convert bytes |
| `.convert_reader(reader)` | `Result<String>` | Build and convert from reader |

---

## Python API

### Basic Conversion

```python
import undocx

markdown = undocx.convert_docx("report.docx")
print(markdown)
```

### Convert from Bytes

```python
import undocx

with open("report.docx", "rb") as f:
    doc_bytes = f.read()

markdown = undocx.convert_docx(doc_bytes)
```

### Skip Images for RAG

```python
import undocx

markdown = undocx.convert_docx("report.docx", image_handling="skip")
```

### Save Images to Directory

```python
import undocx

markdown = undocx.convert_docx("report.docx", image_handling="./extracted_images")
```

### All Options

```python
import undocx

markdown = undocx.convert_docx(
    "report.docx",
    image_handling="skip",
    preserve_whitespace=False,
    html_underline=True,
    html_strikethrough=False,
    strict_reference_validation=True,
)
```

### Batch Processing

```python
import undocx
from pathlib import Path

input_dir = Path("documents")
output_dir = Path("markdown")
output_dir.mkdir(exist_ok=True)

for docx_file in input_dir.glob("*.docx"):
    markdown = undocx.convert_docx(str(docx_file), image_handling="skip")
    output_path = output_dir / f"{docx_file.stem}.md"
    output_path.write_text(markdown)
    print(f"Converted: {docx_file.name}")
```

---

## CLI Usage

```bash
# Print to stdout
undocx report.docx

# Write to file
undocx report.docx output.md

# Extract images to directory
undocx report.docx output.md --images-dir ./images

# Skip images entirely
undocx report.docx --skip-images
```

---

## Error Handling

### Rust Error Handling

```rust
use undocx::{self, Error};

fn safe_convert(path: &str) -> undocx::Result<String> {
    match undocx::convert(path) {
        Ok(md) => Ok(md),
        Err(Error::DocxParse(msg)) => {
            eprintln!("Invalid DOCX: {}", msg);
            Err(Error::DocxParse(msg))
        }
        Err(Error::Io(e)) => {
            eprintln!("File error: {}", e);
            Err(Error::Io(e))
        }
        Err(Error::MissingReference(refs)) => {
            eprintln!("Broken references: {}", refs);
            Err(Error::MissingReference(refs))
        }
        Err(e) => Err(e),
    }
}
```

### Error Types

| Error | Cause |
|-------|-------|
| `undocx::Error::DocxParse` | Invalid or corrupted DOCX file |
| `undocx::Error::Io` | File not found or permission denied |
| `undocx::Error::Conversion` | Internal conversion logic error |
| `undocx::Error::MissingReference` | Broken note/comment reference (strict mode only) |
| `undocx::Error::Zip` | DOCX archive extraction error |
| `undocx::Error::MediaNotFound` | Referenced image not found in DOCX archive |

---

## Architecture

```text
DOCX file → DocxFile (rs-docx) → AstExtractor → DocumentAst → Renderer → Markdown
```

### Pipeline Stages

1. **Parse**: `rs-docx` reads the DOCX ZIP archive and parses XML
2. **Extract**: `AstExtractor` walks body content, producing `DocumentAst`
3. **Render**: `Renderer` serializes `DocumentAst` to output string

### Core Types

| Type | Module | Role |
|------|--------|------|
| `undocx::Converter` | `undocx` | Type alias for default converter |
| `undocx::Builder` | `undocx` | Fluent configuration builder |
| `undocx::DocxToMarkdown<E, R>` | `undocx::converter` | Generic converter with custom components |
| `undocx::core::ast::DocumentAst` | `undocx::core::ast` | Intermediate AST (blocks + references) |
| `undocx::core::ast::BlockNode` | `undocx::core::ast` | `Paragraph` / `TableHtml` / `RawHtml` |
| `undocx::core::ast::ReferenceDefinitions` | `undocx::core::ast` | Footnotes, endnotes, comments |
| `undocx::adapters::docx::AstExtractor` | `undocx::adapters::docx` | Trait: DOCX → AST extraction |
| `undocx::render::Renderer` | `undocx::render` | Trait: AST → output rendering |
| `undocx::converter::ConversionContext` | `undocx::converter` | Shared state during conversion |

---

## Tips for LLM/RAG Pipelines

```rust
use undocx;

fn rag_ingest(docx_bytes: &[u8]) -> undocx::Result<String> {
    // Skip images to reduce token count
    // Use strict mode to catch broken references before indexing
    undocx::builder()
        .skip_images()
        .strict()
        .convert_bytes(docx_bytes)
}
```

- Output is clean Markdown — split on `## ` headers for semantic chunking
- Footnotes and comments are preserved as `[^ref]` for full context
- Average conversion time is 3.3ms per file — suitable for batch processing
- Use `convert_bytes()` for stream-based ingestion from S3 or HTTP
