# undocx

[![Crates.io](https://img.shields.io/crates/v/undocx.svg)](https://crates.io/crates/undocx)
[![PyPI](https://img.shields.io/pypi/v/undocx.svg)](https://pypi.org/project/undocx/)
[![docs.rs](https://docs.rs/undocx/badge.svg)](https://docs.rs/undocx)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Fast, accurate DOCX to Markdown converter built for LLM/RAG pipelines. Written in Rust with Python bindings.

- **Blazing fast**: ~1.6ms per file average (600x faster than pandoc)
- **LLM-optimized**: Clean Markdown output ready for embeddings, chunking, and retrieval
- **Dual interface**: Python library for ML pipelines + CLI for batch processing
- **Full fidelity**: Tables, footnotes, track changes, images, nested lists, and more

## Conversion Demo

<table>
<tr>
<td align="center" width="50%"><strong>DOCX (input)</strong></td>
<td align="center" width="50%"><strong>Markdown (output)</strong></td>
</tr>
<tr>
<td valign="top"><a href="https://github.com/KimSeogyu/undocx/blob/main/docs/undocx_showcase.pdf"><img src="https://raw.githubusercontent.com/KimSeogyu/undocx/main/docs/images/demo-docx.png" alt="DOCX input document"/></a></td>
<td valign="top"><a href="https://github.com/KimSeogyu/undocx/blob/main/docs/undocx_showcase.md"><img src="https://raw.githubusercontent.com/KimSeogyu/undocx/main/docs/images/demo-markdown.png" alt="Converted Markdown output"/></a></td>
</tr>
</table>

> Click images to see full GitHub-rendered files. Headings, bold/italic/underline, tables, nested lists, footnotes, code blocks, track changes -- all converted automatically.

## Install

```bash
pip install undocx          # Python
cargo install undocx        # CLI
```

```toml
# Rust library
[dependencies]
undocx = "0.4"
```

## Quick Start

**CLI**
```bash
undocx report.docx output.md              # convert to file
undocx report.docx                         # print to stdout
undocx report.docx -o out.md --images-dir ./img  # extract images
```

**Python**
```python
import undocx

markdown = undocx.convert_docx("report.docx")           # from path
markdown = undocx.convert_docx(open("r.docx","rb").read())  # from bytes
```

**Rust**
```rust
use undocx::{ConvertOptions, DocxToMarkdown, ImageHandling};

let options = ConvertOptions {
    image_handling: ImageHandling::SaveToDir("./images".into()),
    ..Default::default()
};
let converter = DocxToMarkdown::new(options);
let markdown = converter.convert("report.docx")?;
```

## Supported Features

| Category | Elements |
|----------|----------|
| **Text** | Bold, italic, underline, strikethrough, superscript/subscript |
| **Structure** | Heading 1-9, Title, Subtitle, alignment (center/right) |
| **Lists** | Ordered (decimal, letter, roman, Korean, circled), unordered, nested |
| **Tables** | Colspan, rowspan, nested tables, multi-paragraph cells |
| **Links** | External, internal bookmarks, TOC anchors |
| **Images** | Inline, floating, VML legacy -- base64 embed, save to dir, or skip |
| **Notes** | Footnotes, endnotes, comments (as Markdown `[^ref]`) |
| **Track changes** | Insertions (`<ins>`), deletions (`~~strikethrough~~`) |
| **Other** | Page/column/line breaks, SDT, field codes, bookmarks, symbols |

## Options

| Field | Default | Description |
|-------|---------|-------------|
| `image_handling` | `Inline` | `Inline` / `SaveToDir(path)` / `Skip` |
| `preserve_whitespace` | `false` | Keep original spacing |
| `html_underline` | `true` | `<u>` tags for underline |
| `html_strikethrough` | `false` | `<s>` tags instead of `~~` |
| `strict_reference_validation` | `false` | Fail on broken note/comment refs |

## Advanced: Custom Pipeline

Replace the default extractor or renderer:

```rust
let converter = DocxToMarkdown::with_components(
    ConvertOptions::default(),
    MyExtractor,    // impl AstExtractor
    MyRenderer,     // impl Renderer
);
```

See [docs/API_POLICY.md](docs/API_POLICY.md) for stability guarantees on these traits.

## Benchmarks

Measured on the included test corpus (Korean financial/legal documents):

| Metric | Value |
|--------|-------|
| Avg per file | **1.65 ms** |
| Min | 0.43 ms |
| Max | 6.08 ms |
| Throughput | ~600 files/sec |

Run locally: `./scripts/run_perf_benchmark.sh`

## Comparison

| Feature | undocx | pandoc | python-docx | mammoth |
|---------|--------|--------|-------------|---------|
| Language | Rust | Haskell | Python | JS/Python |
| Speed | ~1.6ms/file | ~1s/file | ~200ms/file | ~100ms/file |
| Tables (colspan/rowspan) | Yes | Partial | Read-only | Yes |
| Track changes | Yes | Yes | No | No |
| Footnotes/Endnotes | Yes | Yes | No | Yes |
| Comments | Yes | No | No | No |
| VML legacy images | Yes | No | No | No |
| Korean numbering | Yes | No | No | No |
| Python API | Yes | CLI only | Yes | Yes |
| Rust API | Yes | No | No | No |
| Binary size | ~4 MB | ~120 MB | N/A | N/A |

## For LLM/RAG Users

undocx is designed for document preprocessing in AI pipelines:

```python
import undocx

# Skip images for text-only RAG ingestion
md = undocx.convert_docx("report.docx", image_handling="skip")

# Process bytes from S3, HTTP, etc.
md = undocx.convert_docx(doc_bytes, image_handling="skip")
```

**Tips for RAG pipelines:**
- Use `image_handling="skip"` to reduce token count
- Output is clean Markdown — split on `## ` headers for semantic chunking
- Footnotes and comments are preserved as `[^ref]` for full context

## Development

```bash
cargo test --all-features                                  # test
cargo clippy --all-features --tests -- -D warnings         # lint
./scripts/run_perf_benchmark.sh                            # bench
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and guidelines.

## License

MIT — see [LICENSE](LICENSE)
