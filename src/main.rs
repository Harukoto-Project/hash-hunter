mod cli;
mod detect;
mod error;
mod git;
mod report;

use clap::Parser;

use crate::cli::Args;

fn main() {
    let _args = Args::parse();
}
