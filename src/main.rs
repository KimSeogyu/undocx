//! CLI for undocx - DOCX to Markdown converter

use clap::Parser;
use undocx::{ConvertOptions, DocxToMarkdown, ImageHandling};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input DOCX file path
    input: PathBuf,

    /// Output Markdown file path (optional, prints to stdout if not specified)
    output: Option<PathBuf>,

    /// Directory to extract images to (if not set, images are embedded/inline)
    #[arg(long)]
    images_dir: Option<PathBuf>,

    /// Skip extracting images
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
