// src/main.rs
/*
 * Main executable for TPUTestnetAIChainUltra
 */

use clap::Parser;
use tputestnetaichainultra::{Result, run};

#[derive(Parser)]
#[command(version, about = "TPUTestnetAIChainUltra - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
