//! Command-line tool to decompresses a file in ZX0 format.

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    /// Compressed file to read from
    input: PathBuf,
    /// File to write the decompressed data to
    output: PathBuf,
}

fn execute(args: Cli) -> std::io::Result<()> {
    let mut source = std::fs::File::open(args.input)?;
    let content = zx0decompress::decompress(&mut source)?;
    std::fs::write(args.output, content)
}

fn main() {
    let args = Cli::parse();
    if let Err(e) = execute(args) {
        eprintln!("Failed: {e}");
        std::process::exit(1);
    }
}
