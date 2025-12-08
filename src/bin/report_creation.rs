use clap::Parser;
use std::{fs, path::PathBuf};

use ReportCreation as reportcreation;

/// Generate a PDF file from an existing Typst document.
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Path to the Typst file that should be compiled.
    #[arg(value_name = "INPUT.typ")] 
    input: PathBuf,

    /// Output path for the generated PDF. Defaults to replacing the extension with `.pdf`.
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let source = fs::read_to_string(&cli.input)?;
    let output_path = cli
        .output
        .clone()
        .unwrap_or_else(|| cli.input.with_extension("pdf"));

    let pdf_bytes = reportcreation::compile_pdf(&source, &cli.input);
    fs::write(&output_path, &pdf_bytes)?;

    println!("PDF written to {}", output_path.display());

    Ok(())
}
