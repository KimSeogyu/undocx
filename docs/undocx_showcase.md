# Document Parsing Showcase

This document demonstrates <strong>various formatting features</strong> that `undocx` can parse into clean Markdown. It includes <strong>bold</strong>, <em>italic</em>, <u>underlined</u>, ~~strikethrough~~, and <strong><em>bold italic</em></strong> text. You can even combine <strong><u>bold + underline</u></strong> or use <sub>subscript</sub> and <sup>superscript</sup>.

## Key Features

- Full <strong>OOXML</strong> parsing with <em>structural fidelity</em>

  - Handles nested lists and complex tables

  - Preserves semantic heading hierarchy

- Inline code: `cargo run --release`

- Hyperlink support: [<u>github.com/undocx</u>](https://github.com/undocx)

- Footnote extraction<sup>[^1]</sup> with full reference mapping

## Benchmark Results

<table>
  <tr>
    <td><strong>Parser</strong></td>
    <td><div style="text-align: center;"><strong>Headings</strong></div></td>
    <td><div style="text-align: center;"><strong>Tables</strong></div></td>
    <td><div style="text-align: center;"><strong>Lists</strong></div></td>
    <td><div style="text-align: center;"><strong>Overall</strong></div></td>
  </tr>
  <tr>
    <td>undocx (Rust)</td>
    <td><div style="text-align: center;"><strong>99.2%</strong></div></td>
    <td><div style="text-align: center;"><strong>97.8%</strong></div></td>
    <td><div style="text-align: center;"><strong>98.5%</strong></div></td>
    <td><div style="text-align: center;"><strong>98.4%</strong></div></td>
  </tr>
  <tr>
    <td>Pandoc</td>
    <td><div style="text-align: center;">95.1%</div></td>
    <td><div style="text-align: center;">82.3%</div></td>
    <td><div style="text-align: center;">90.7%</div></td>
    <td><div style="text-align: center;">89.4%</div></td>
  </tr>
  <tr>
    <td>Mammoth.js</td>
    <td><div style="text-align: center;">91.0%</div></td>
    <td><div style="text-align: center;">78.6%</div></td>
    <td><div style="text-align: center;">85.2%</div></td>
    <td><div style="text-align: center;">84.9%</div></td>
  </tr>
  <tr>
    <td>docx2md</td>
    <td><div style="text-align: center;">88.4%</div></td>
    <td><div style="text-align: center;">71.2%</div></td>
    <td><div style="text-align: center;">80.1%</div></td>
    <td><div style="text-align: center;">79.9%</div></td>
  </tr>
</table>

<em>Accuracy measured on 500 real-world documents</em><sup>[^2]</sup>

## Quick Start

1. Install via Cargo: `cargo install undocx`

2. Convert a file: `undocx report.docx -o report.md`

3. Verify output matches the original document structure

### What Users Say

“<em>We switched from Pandoc to undocx and our conversion accuracy jumped from 89% to 98%. The table parsing alone saved us hours of manual cleanup.</em>”

<div style="text-align: right;">— <strong>Engineering Team, Acme Corp</strong></div>

### Code Block Example

```
use undocx::Parser;
 
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new("document.docx")?;
    let markdown = parser.to_markdown()?;
    std::fs::write("output.md", &markdown)?;
    println!("Converted {} elements", parser.stats().total);
    Ok(())
}
```

<div style="text-align: center;">Built with ❤ in Rust. <em>MIT Licensed</em> · [<u>Documentation</u>](https://undocx.dev/docs) · [<u>GitHub</u>](https://github.com/undocx)</div>

---

[^1]: Pandoc supports .docx reading but only produces approximate Markdown. undocx aims for structural fidelity.
[^2]: Benchmarked against 500 real-world documents from government and academic sources.
