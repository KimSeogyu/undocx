//! CLI for undocx - DOCX to Markdown converter

use clap::Parser;
use undocx::{ConvertOptions, DocxToMarkdown, ImageHandling};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Convert DOCX files to Markdown",
    long_about = "undocx converts Microsoft Word (.docx) files to clean Markdown.\n\n\
        Supports headings, lists (decimal/letter/roman/Korean), tables with \
        cell merges, footnotes, endnotes, comments, track changes, images, \
        hyperlinks, bookmarks, and more.\n\n\
        Images are embedded as base64 by default. Use --images-dir to extract \
        them to a directory, or --skip-images to omit them entirely.",
    after_help = "EXAMPLES:\n  \
        undocx report.docx                        Print Markdown to stdout\n  \
        undocx report.docx output.md              Write to file\n  \
        undocx report.docx out.md --images-dir img  Extract images to ./img/\n  \
        undocx report.docx --skip-images          Omit all images"
)]
struct Args {
    /// Input .docx file
    input: PathBuf,

    /// Output .md file (prints to stdout if omitted)
    output: Option<PathBuf>,

    /// Save images to this directory instead of embedding as base64
    #[arg(long, value_name = "DIR")]
    images_dir: Option<PathBuf>,

    /// Omit images from the output entirely
    #[arg(long)]
    skip_images: bool,
}

fn main() {
    let args = Args::parse();

    let image_handling = if args.skip_images {
        ImageHandling::Skip
    } else if let Some(dir) = args.images_dir {
        ImageHandling::SaveToDir(dir)
    } else {
        ImageHandling::Inline
    };

    let options = ConvertOptions {
        image_handling,
        ..Default::default()
    };

    let converter = DocxToMarkdown::new(options);

    match converter.convert(&args.input) {
        Ok(markdown) => {
            if let Some(output) = args.output {
                if let Err(e) = std::fs::write(&output, &markdown) {
                    eprintln!("Error writing output: {}", e);
                    std::process::exit(1);
                }
                println!("Successfully converted to {:?}", output);
            } else {
                println!("{}", markdown);
            }
        }
        Err(e) => {
            eprintln!("Error converting DOCX: {}", e);
            std::process::exit(1);
        }
    }
}
