mod models;
mod parser;

use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

use crate::models::ScrapboxExport;
use crate::parser::parse_page;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input Scrapbox JSON file
    input: PathBuf,

    /// Output directory for Markdown files
    #[arg(short, long, default_value = "out")]
    outdir: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Ensure output directory exists
    if !args.outdir.exists() {
        fs::create_dir_all(&args.outdir)
            .with_context(|| format!("Failed to create directory {:?}", args.outdir))?;
    }

    // Read and parse JSON
    let content = fs::read_to_string(&args.input)
        .with_context(|| format!("Failed to read file {:?}", args.input))?;

    let data: ScrapboxExport =
        serde_json::from_str(&content).with_context(|| "Failed to parse JSON")?;

    println!("Found {} pages. Starting conversion...", data.pages.len());

    for page in data.pages {
        let markdown = parse_page(&page);
        let safe_title = sanitize_filename(&page.title);
        let output_path = args.outdir.join(format!("{safe_title}.md"));

        fs::write(&output_path, markdown)
            .with_context(|| format!("Failed to write to {output_path:?}"))?;
    }

    println!("Conversion completed! Files are in {:?}", args.outdir);
    Ok(())
}

fn sanitize_filename(name: &str) -> String {
    // Replace characters that are invalid in filenames
    name.chars()
        .map(|c| match c {
            '/' | '\\' | '<' | '>' | ':' | '"' | '|' | '?' | '*' => '_',
            _ => c,
        })
        .collect()
}
