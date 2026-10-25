mod cli;

use clap::Parser;

use hash_hunter::detect;
use hash_hunter::error::AppError;
use hash_hunter::report;
use hash_hunter::scan;

use crate::cli::Args;

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
    let scored = scan::collect(&repo, args.branch.as_deref(), &args.only, &args.exclude)?;
    report::print_ranking(&scored, args.top, args.min_score);
    Ok(())
}
