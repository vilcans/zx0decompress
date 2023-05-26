//! Command-line tool to decompresses a file in ZX0 format.

use clap::Parser;
use std::path::PathBuf;
use zx0decompress::{DecompressError, Settings};

#[derive(Parser, Debug)]
struct Cli {
    /// Treat input as old (v1) file format
    #[arg(short, long)]
    classic: bool,
    /// Maximum number of decompressed bytes to write
    #[arg(short, long, value_name = "SIZE")]
    max_output_size: Option<usize>,
    /// Compressed file to read from
    input: PathBuf,
    /// File to write the decompressed data to
    output: PathBuf,
}

fn execute(args: Cli) -> Result<(), DecompressError> {
    let mut source = std::fs::File::open(args.input)?;
    let settings = Settings {
        classic_mode: args.classic,
        max_output_size: args.max_output_size.unwrap_or(usize::MAX),
    };
    let content = zx0decompress::decompress_with_settings(&mut source, settings)?;
    std::fs::write(args.output, content)?;
    Ok(())
}

fn main() {
    let args = Cli::parse();
    if let Err(e) = execute(args) {
        eprintln!("Failed: {e}");
        std::process::exit(1);
    }
}
