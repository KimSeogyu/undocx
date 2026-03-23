# undocx API Guide

This guide walks through every way to use undocx — from a single function
call to a fully custom conversion pipeline. Pick the section that matches
your use case; each one is self-contained with runnable examples.

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

If Markdown isn't your target format, you can replace the renderer entirely.
Implement the `Renderer` trait to produce plain text, HTML, JSON, or anything else:

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

The other side of the pipeline is extraction — how DOCX body elements
become AST nodes. Implement `AstExtractor` to filter, transform, or
enrich the document before rendering:

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

---

## Recipes

### Track Changes and Comments

Track changes and comments are converted **automatically** — no configuration needed.
Insertions become `<ins>` tags, deletions become `~~strikethrough~~`, and comments
become Markdown footnote references `[^cID]`.

```rust
use undocx;

fn main() -> undocx::Result<()> {
    // Track changes and comments are included by default.
    // No special options required.
    let md = undocx::convert("contract_with_redlines.docx")?;

    // Output contains:
    //   Insertions:  <ins>new text</ins>
    //   Deletions:   ~~removed text~~
    //   Comments:    Some text[^c1]
    //
    // Comment definitions appear at the end:
    //   ---
    //   [^c1]: Reviewer's comment text here
    println!("{}", md);
    Ok(())
}
```

Use `strict()` to ensure all comment and note references resolve correctly:

```rust
use undocx;

fn main() -> undocx::Result<()> {
    let md = undocx::builder()
        .strict()  // fails with Error::MissingReference if any comment/note is broken
        .convert("contract_with_redlines.docx")?;
    println!("{}", md);
    Ok(())
}
```

Python:

```python
import undocx

# Track changes and comments are always included
markdown = undocx.convert_docx("contract_with_redlines.docx")

# With strict validation
markdown = undocx.convert_docx(
    "contract_with_redlines.docx",
    strict_reference_validation=True,
)
```

### Footnotes and Endnotes

Footnotes and endnotes are converted **automatically** into Markdown reference-style
links. Footnotes use `[^1]`, `[^2]`, etc. Endnotes use `[^en1]`, `[^en2]`, etc.
Definitions are appended after a `---` separator at the end of the document.

Complete script that converts a DOCX and verifies footnote output:

```rust
use undocx;

fn main() -> undocx::Result<()> {
    // Footnotes and endnotes are always extracted — no configuration needed
    let md = undocx::convert("research_paper.docx")?;

    // Output format:
    //   Body text with a footnote reference[^1] and an endnote[^en1].
    //
    //   ---
    //   [^1]: This is the footnote text.
    //   [^en1]: This is the endnote text.

    // Verify footnotes are present
    assert!(md.contains("[^1]"), "Footnote reference missing");
    assert!(md.contains("---"), "Reference separator missing");

    std::fs::write("output.md", &md)?;
    println!("Converted with footnotes ({} bytes)", md.len());
    Ok(())
}
```

Use strict mode to catch broken footnote/endnote references:

```rust
use undocx;

fn convert_with_validated_notes(path: &str) -> undocx::Result<String> {
    // strict() causes Error::MissingReference if any [^N] cannot be resolved
    undocx::builder()
        .strict()
        .convert(path)
}
```

Python:

```python
import undocx

# Footnotes and endnotes are always included
markdown = undocx.convert_docx("research_paper.docx")

# Validate all references resolve
markdown = undocx.convert_docx(
    "research_paper.docx",
    strict_reference_validation=True,
)

# Output contains [^1], [^en1] in body and definitions after ---
print(markdown)
```

### VML Legacy Images

undocx handles legacy VML (Vector Markup Language) images alongside modern DrawingML
images. To **include** VML images in the output, use `inline_images()` (base64 embed)
or `save_images_to()` (save to directory). VML images are extracted from the DOCX
archive automatically.

Embed VML images as inline base64:

```rust
use undocx;

fn main() -> undocx::Result<()> {
    // inline_images() is the default — VML images are embedded as base64
    let md = undocx::convert("legacy_document.docx")?;

    // Output: ![](data:image/png;base64,iVBORw0KGgo...)
    assert!(md.contains("data:image/"), "Image should be embedded");
    println!("{}", md);
    Ok(())
}
```

Save VML images to a directory:

```rust
use undocx;

fn main() -> undocx::Result<()> {
    let md = undocx::builder()
        .save_images_to("./extracted_images")  // VML images saved here
        .convert("legacy_document.docx")?;

    // Output: ![](extracted_images/image1.png)
    // Files are written to ./extracted_images/
    println!("{}", md);
    Ok(())
}
```

Python:

```python
import undocx

# Inline base64 (default) — includes VML images
markdown = undocx.convert_docx("legacy_document.docx")
# Output: ![](data:image/png;base64,...)

# Save to directory — includes VML images as files
markdown = undocx.convert_docx(
    "legacy_document.docx",
    image_handling="./extracted_images",
)
# Output: ![](extracted_images/image1.png)
```

### High-Throughput Concurrent Processing

`undocx::Converter` is `Send + Sync` — safe to share across threads. Build one
converter and use it from multiple threads with Rayon or Tokio for high-throughput
batch processing.

Parallel processing with Rayon:

```rust
use undocx::Converter;
use rayon::prelude::*;
use std::path::PathBuf;

fn batch_convert(files: Vec<PathBuf>) -> Vec<undocx::Result<String>> {
    let converter = Converter::builder()
        .skip_images()
        .strict()
        .build();

    // Process thousands of files in parallel
    files.par_iter()
        .map(|path| converter.convert(path))
        .collect()
}
```

Async processing with Tokio:

```rust
use undocx::Converter;
use std::sync::Arc;

async fn convert_many(files: Vec<String>) -> Vec<undocx::Result<String>> {
    let converter = Arc::new(
        Converter::builder().skip_images().build()
    );

    let mut handles = Vec::new();
    for path in files {
        let conv = Arc::clone(&converter);
        handles.push(tokio::task::spawn_blocking(move || {
            conv.convert(&path)
        }));
    }

    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.await.unwrap());
    }
    results
}
```

Python batch processing:

```python
import undocx
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor

def batch_convert(input_dir: str, output_dir: str, max_workers: int = 8):
    input_path = Path(input_dir)
    output_path = Path(output_dir)
    output_path.mkdir(exist_ok=True)

    docx_files = list(input_path.glob("*.docx"))

    def convert_one(docx_file):
        markdown = undocx.convert_docx(str(docx_file), image_handling="skip")
        out = output_path / f"{docx_file.stem}.md"
        out.write_text(markdown)
        return docx_file.name

    with ThreadPoolExecutor(max_workers=max_workers) as pool:
        for name in pool.map(convert_one, docx_files):
            print(f"Converted: {name}")

batch_convert("./documents", "./markdown_output")
```

### Zero Data Loss Conversion

To ensure **full fidelity** — no data is lost during conversion — use the default
settings. undocx preserves all document elements by default:

```rust
use undocx;

fn full_fidelity_convert(path: &str) -> undocx::Result<String> {
    // Default settings preserve everything:
    //   - Images: embedded as inline base64 (no data loss)
    //   - Footnotes/endnotes: converted to [^ref] definitions
    //   - Comments: converted to [^cID] definitions
    //   - Track changes: insertions as <ins>, deletions as ~~text~~
    //   - Tables: full HTML with colspan/rowspan
    //   - Lists: all numbering styles (decimal, roman, Korean, etc.)
    //   - Links: external URLs and internal bookmark anchors
    //   - Formatting: bold, italic, underline (<u>), strikethrough
    //   - Structure: headings H1-H9, title, subtitle, alignment
    //   - Other: page breaks, bookmarks, symbols, field codes
    let md = undocx::builder()
        .inline_images()    // embed images as base64 (default)
        .html_underline(true)       // preserve underlines as <u> (default)
        .html_strikethrough(false)  // use ~~ for strikethrough (default)
        .strict()           // fail if any reference is broken
        .convert(path)?;

    Ok(md)
}
```

Verify completeness by checking the output:

```rust
use undocx;

fn convert_and_verify(path: &str) -> undocx::Result<()> {
    let md = undocx::builder()
        .inline_images()
        .strict()
        .convert(path)?;

    // Verify key elements are present
    let has_images = md.contains("data:image/");
    let has_footnotes = md.contains("[^");
    let has_tables = md.contains("<table");
    let has_headings = md.contains("# ");

    println!("Images: {}, Footnotes: {}, Tables: {}, Headings: {}",
        has_images, has_footnotes, has_tables, has_headings);

    std::fs::write("full_output.md", &md)?;
    Ok(())
}
```

Python full-fidelity conversion:

```python
import undocx

# Default settings = zero data loss
# All elements preserved: images, footnotes, comments, track changes, tables
markdown = undocx.convert_docx(
    "complex_document.docx",
    image_handling="inline",              # embed images (default)
    html_underline=True,                  # preserve underlines (default)
    strict_reference_validation=True,     # ensure no broken references
)

print(f"Converted: {len(markdown)} characters")
print(f"Images: {'data:image/' in markdown}")
print(f"Footnotes: {'[^' in markdown}")
print(f"Tables: {'<table' in markdown}")
```
