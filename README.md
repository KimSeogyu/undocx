# undocx

[![Crates.io](https://img.shields.io/crates/v/undocx.svg)](https://crates.io/crates/undocx)
[![PyPI](https://img.shields.io/pypi/v/undocx.svg)](https://pypi.org/project/undocx/)
[![docs.rs](https://docs.rs/undocx/badge.svg)](https://docs.rs/undocx)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

**Fast, accurate DOCX to Markdown converter built for LLM/RAG pipelines.** Written in Rust with Python bindings.

- **16.5x faster than pandoc** — 3.3ms per file average
- **LLM-optimized** — Clean Markdown output ready for embeddings, chunking, and retrieval
- **Full fidelity** — Tables, footnotes, track changes, images, nested lists, and more

[For Humans](#for-humans) • [For Agents](#for-agents) • [Benchmarks](#benchmarks) • [Features](#supported-features) • [Contributing](CONTRIBUTING.md)

---

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

> Click images to see full GitHub-rendered files.

## Benchmarks

Measured on 39 DOCX files × 10 iterations ([reproduce it yourself](examples/benchmark_comparison.py)):

| Tool | Avg (ms) | Median (ms) | Min (ms) | Max (ms) |
|------|----------|-------------|----------|----------|
| **undocx** | **3.34** | **3.22** | **2.89** | **5.46** |
| markitdown | 18.25 | 17.45 | 14.63 | 41.81 |
| pandoc | 55.08 | 54.11 | 40.31 | 69.51 |

**undocx is 16.5x faster than pandoc and 5.5x faster than markitdown.**

| Feature | undocx | pandoc | markitdown |
|---------|--------|--------|------------|
| Language | Rust | Haskell | Python |
| Speed (avg) | 3.3ms/file | 55ms/file | 18ms/file |
| Tables (colspan/rowspan) | Yes | Partial | Yes |
| Track changes | Yes | Yes | No |
| Footnotes/Endnotes | Yes | Yes | No |
| Comments | Yes | No | No |
| VML legacy images | Yes | No | No |
| Korean numbering | Yes | No | No |
| Python API | Yes | CLI only | Yes |
| Rust API | Yes | No | No |

---

## For Humans

Install and convert — that's it.

```bash
pip install undocx          # Python
cargo install undocx        # CLI
```

**CLI**
```bash
undocx report.docx output.md              # convert to file
undocx report.docx                         # print to stdout
undocx report.docx -o out.md --images-dir ./img  # extract images
```

**Python**
```python
import undocx

markdown = undocx.convert_docx("report.docx")
```

---

## For Agents

Designed for document preprocessing in LLM/RAG pipelines.

**Python — RAG ingestion**
```python
import undocx

# Skip images for text-only RAG ingestion
md = undocx.convert_docx("report.docx", image_handling="skip")

# Process bytes from S3, HTTP, or any byte stream
md = undocx.convert_docx(doc_bytes, image_handling="skip")
```

**Rust — Custom pipeline**
```rust
use undocx::{ConvertOptions, DocxToMarkdown, ImageHandling};

let options = ConvertOptions {
    image_handling: ImageHandling::Skip,  // optimal for RAG
    ..Default::default()
};
let converter = DocxToMarkdown::new(options);
let markdown = converter.convert("report.docx")?;
```

**Rust — Pluggable architecture**
```rust
let converter = DocxToMarkdown::with_components(
    ConvertOptions::default(),
    MyExtractor,    // impl AstExtractor
    MyRenderer,     // impl Renderer
);
```

See [docs/API_POLICY.md](docs/API_POLICY.md) for stability guarantees on these traits.

```toml
# Cargo.toml
[dependencies]
undocx = "0.4"
```

**Tips for RAG pipelines:**
- Use `image_handling="skip"` to reduce token count
- Output is clean Markdown — split on `## ` headers for semantic chunking
- Footnotes and comments are preserved as `[^ref]` for full context

---

## Supported Features

| Category | Elements |
|----------|----------|
| **Text** | Bold, italic, underline, strikethrough, superscript/subscript |
| **Structure** | Heading 1-9, Title, Subtitle, alignment (center/right) |
| **Lists** | Ordered (decimal, letter, roman, Korean, circled), unordered, nested |
| **Tables** | Colspan, rowspan, nested tables, multi-paragraph cells |
| **Links** | External, internal bookmarks, TOC anchors |
| **Images** | Inline, floating, VML legacy — base64 embed, save to dir, or skip |
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

## Development

```bash
cargo test --all-features                                  # test
cargo clippy --all-features --tests -- -D warnings         # lint
python examples/benchmark_comparison.py ./tests/pandoc 10  # bench
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and guidelines.

## License

MIT — see [LICENSE](LICENSE)
