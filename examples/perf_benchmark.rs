use undocx::{ConvertOptions, DocxToMarkdown, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Debug, Clone)]
struct PerfConfig {
    input_dir: PathBuf,
    iterations: usize,
    max_files: usize,
}

impl Default for PerfConfig {
    fn default() -> Self {
        Self {
            input_dir: PathBuf::from("./tests/aaa"),
            iterations: 3,
            max_files: 5,
        }
    }
}

fn parse_args() -> PerfConfig {
    let mut cfg = PerfConfig::default();
    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input-dir" => {
                if let Some(value) = args.next() {
                    cfg.input_dir = PathBuf::from(value);
                }
            }
            "--iterations" => {
                if let Some(value) = args.next() {
                    if let Ok(parsed) = value.parse::<usize>() {
                        cfg.iterations = parsed.max(1);
                    }
                }
            }
            "--max-files" => {
                if let Some(value) = args.next() {
                    if let Ok(parsed) = value.parse::<usize>() {
                        cfg.max_files = parsed.max(1);
                    }
                }
            }
            _ => {}
        }
    }

    cfg
}

fn list_docx_files(dir: &Path, max_files: usize) -> Result<Vec<PathBuf>> {
    let mut files = fs::read_dir(dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file()
                && path
                    .extension()
                    .and_then(|e| e.to_str())
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("docx"))
        })
        .collect::<Vec<_>>();
    files.sort();
    files.truncate(max_files);
    Ok(files)
}

fn main() -> Result<()> {
    let cfg = parse_args();
    let files = list_docx_files(&cfg.input_dir, cfg.max_files)?;

    if files.is_empty() {
        println!("no-docx-files-found input_dir={}", cfg.input_dir.display());
        return Ok(());
    }

    let converter = DocxToMarkdown::new(ConvertOptions::default());
    let mut samples = Vec::new();

    let overall_start = Instant::now();
    for _ in 0..cfg.iterations {
        for file in &files {
            let start = Instant::now();
            let _ = converter.convert(file)?;
            samples.push(start.elapsed().as_secs_f64() * 1000.0);
        }
    }
    let overall_ms = overall_start.elapsed().as_secs_f64() * 1000.0;

    let count = samples.len() as f64;
    let total_ms: f64 = samples.iter().sum();
    let avg_ms = total_ms / count;
    let min_ms = samples.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_ms = samples.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    println!(
        "{{\"input_dir\":\"{}\",\"iterations\":{},\"files\":{},\"samples\":{},\"avg_ms\":{:.3},\"min_ms\":{:.3},\"max_ms\":{:.3},\"total_ms\":{:.3},\"overall_ms\":{:.3}}}",
        cfg.input_dir.display(),
        cfg.iterations,
        files.len(),
        samples.len(),
        avg_ms,
        min_ms,
        max_ms,
        total_ms,
        overall_ms
    );

    Ok(())
}
