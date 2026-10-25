mod cli;
mod detect;
mod error;
mod git;
mod report;

use clap::Parser;

use crate::cli::Args;
use crate::error::AppError;

fn main() {
    if let Err(err) = run(Args::parse()) {
        eprintln!("error[{}]: {err}", err.code());
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<(), AppError> {
    if args.list_detectors {
        report::print_detector_list(&detect::list_ids());
        return Ok(());
    }

    let repo = args.repo.expect("repo is required unless --list-detectors");
    let detectors = detect::select(&args.only, &args.exclude)?;
    let workspace = git::Workspace::open(&repo)?;
    let commits = git::list_commits(workspace.path(), args.branch.as_deref())?;
    let scored: Vec<_> = commits
        .iter()
        .map(|commit| detect::score_hash(&commit.hash, &detectors))
        .collect();
    report::print_ranking(&scored, args.top, args.min_score);
    Ok(())
}
