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

The simplest way to convert a DOCX file:

```rust
fn main() -> undocx::Result<()> {
    let md = undocx::convert("report.docx")?;
    println!("{}", md);
    Ok(())
}
```

### Convert from Bytes

Useful when reading from S3, HTTP responses, or databases:

```rust
fn convert_from_s3(bytes: &[u8]) -> undocx::Result<String> {
    undocx::convert_bytes(bytes)
}
```

### Convert from Reader

Any type implementing `Read + Seek` works:

```rust
use std::fs::File;

fn main() -> undocx::Result<()> {
    let file = File::open("report.docx").unwrap();
    let md = undocx::convert_reader(file)?;
    println!("{}", md);
    Ok(())
}
```

### Builder Pattern

Configure conversion options with a fluent API:

```rust
// Skip images for text-only RAG pipelines
let md = undocx::builder()
    .skip_images()
    .convert("report.docx")?;
```

```rust
// Save images to a directory
let md = undocx::builder()
    .save_images_to("./images")
    .convert("report.docx")?;
```

```rust
// Strict mode: fail on broken references
let md = undocx::builder()
    .skip_images()
    .strict()
    .convert("report.docx")?;
```

```rust
// All formatting options
let md = undocx::builder()
    .skip_images()
    .preserve_whitespace()
    .html_underline(false)
    .html_strikethrough(true)
    .strict()
    .convert("report.docx")?;
```

### Reusable Converter

Build once, convert many files:

```rust
let converter = undocx::Converter::builder()
    .skip_images()
    .strict()
    .build();

let md1 = converter.convert("a.docx")?;
let md2 = converter.convert("b.docx")?;
let md3 = converter.convert_bytes(&bytes)?;
```

### Custom Pipeline

Implement custom extractors or renderers for specialized output:

```rust
use undocx::adapters::docx::AstExtractor;
use undocx::converter::ConversionContext;
use undocx::core::ast::{BlockNode, DocumentAst};
use undocx::render::Renderer;
use undocx::{ConvertOptions, DocxToMarkdown, Result};
use rs_docx::document::BodyContent;

struct PlainTextRenderer;

impl Renderer for PlainTextRenderer {
    fn render(&self, document: &DocumentAst) -> Result<String> {
        let mut output = String::new();
        for block in &document.blocks {
            match block {
                BlockNode::Paragraph(text) => {
                    output.push_str(text);
                    output.push('\n');
                }
                BlockNode::TableHtml(html) => {
                    output.push_str(html);
                    output.push('\n');
                }
                BlockNode::RawHtml(html) => {
                    output.push_str(html);
                    output.push('\n');
                }
            }
        }
        Ok(output)
    }
}

let converter = DocxToMarkdown::with_components(
    ConvertOptions::default(),
    undocx::adapters::docx::DocxExtractor,  // default extractor
    PlainTextRenderer,                       // custom renderer
);
let output = converter.convert("report.docx")?;
```

---

## Configuration Reference

### ImageHandling

| Value | Description | Use Case |
|-------|-------------|----------|
| `ImageHandling::Inline` | Embed as base64 data URIs (default) | Full document conversion |
| `ImageHandling::SaveToDir(path)` | Save to directory, reference by path | Web publishing |
| `ImageHandling::Skip` | Omit images entirely | RAG pipelines, text-only |

### ConvertOptions

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `image_handling` | `ImageHandling` | `Inline` | How to handle embedded images |
| `preserve_whitespace` | `bool` | `false` | Keep original whitespace |
| `html_underline` | `bool` | `true` | Use `<u>` for underline |
| `html_strikethrough` | `bool` | `false` | Use `<s>` instead of `~~` |
| `strict_reference_validation` | `bool` | `false` | Fail on broken note/comment refs |

### Builder Methods

```rust
undocx::builder()
    // Image handling (mutually exclusive, last wins)
    .skip_images()                  // ImageHandling::Skip
    .inline_images()                // ImageHandling::Inline (default)
    .save_images_to("./images")     // ImageHandling::SaveToDir

    // Formatting
    .preserve_whitespace()          // preserve_whitespace = true
    .html_underline(false)          // html_underline = false
    .html_strikethrough(true)       // html_strikethrough = true
    .strict()                       // strict_reference_validation = true

    // Terminal: build or convert
    .build()                        // → Converter (reusable)
    .convert("file.docx")           // → Result<String>
    .convert_bytes(&bytes)          // → Result<String>
    .convert_reader(reader)         // → Result<String>
```

---

## Python API

### Basic Conversion

```python
import undocx

# From file path
markdown = undocx.convert_docx("report.docx")
print(markdown)
```

### From Bytes

```python
import undocx

# From any byte source (S3, HTTP, database)
with open("report.docx", "rb") as f:
    doc_bytes = f.read()

markdown = undocx.convert_docx(doc_bytes)
```

### With Options

```python
import undocx

# Skip images for RAG/LLM pipelines
markdown = undocx.convert_docx(
    "report.docx",
    image_handling="skip",
)
```

```python
import undocx

# Save images to directory
markdown = undocx.convert_docx(
    "report.docx",
    image_handling="./extracted_images",
)
```

```python
import undocx

# All options
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

# Skip images
undocx report.docx --skip-images
```

---

## Error Handling

### Rust

```rust
use undocx::{Error, Result};

fn safe_convert(path: &str) -> Result<String> {
    match undocx::convert(path) {
        Ok(md) => Ok(md),
        Err(Error::DocxParse(msg)) => {
            eprintln!("Invalid DOCX file: {}", msg);
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
| `Error::DocxParse` | Invalid or corrupted DOCX file |
| `Error::Io` | File system error (not found, permission denied) |
| `Error::Conversion` | Internal conversion logic error |
| `Error::MissingReference` | Broken footnote/endnote/comment reference (strict mode) |
| `Error::Zip` | DOCX archive extraction error |
| `Error::MediaNotFound` | Referenced image not found in DOCX archive |

---

## Architecture

```
DOCX file → DocxFile (rs-docx) → AstExtractor → DocumentAst → Renderer → Markdown
```

### Pipeline Stages

1. **Parse**: `rs-docx` reads the DOCX ZIP archive and parses XML
2. **Extract**: `AstExtractor` walks the body content, producing `DocumentAst`
3. **Render**: `Renderer` serializes `DocumentAst` to output string

### Key Types

| Type | Role |
|------|------|
| `Converter` | Type alias for `DocxToMarkdown` with default components |
| `Builder` | Fluent configuration builder |
| `DocxToMarkdown<E, R>` | Generic converter, parameterized by extractor and renderer |
| `DocumentAst` | Intermediate representation (blocks + references) |
| `BlockNode` | `Paragraph` \| `TableHtml` \| `RawHtml` |
| `ReferenceDefinitions` | Footnotes, endnotes, comments |
| `AstExtractor` | Trait for customizing DOCX → AST extraction |
| `Renderer` | Trait for customizing AST → output rendering |
| `ConversionContext` | Shared state during conversion (styles, numbering, images) |

### Extensibility

Implement `AstExtractor` to customize parsing, or `Renderer` to produce
different output formats. Both are public traits with stable API contracts.
See [API_POLICY.md](API_POLICY.md) for stability guarantees.

---

## Tips for LLM/RAG Pipelines

- Use `.skip_images()` or `image_handling="skip"` to reduce token count
- Split output on `## ` headers for semantic chunking
- Footnotes and comments are preserved as `[^ref]` for full context
- Average conversion time is 3.3ms per file — suitable for batch processing
- Use `.strict()` to catch broken references before indexing
