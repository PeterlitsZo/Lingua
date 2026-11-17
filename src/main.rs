mod cli;
mod core;

use std::{process::ExitCode, time::Instant};

use crate::cli::Cli;

fn main() -> ExitCode {
    let start = Instant::now();
    let cli = Cli::parse();
    let mut timings = core::timing::Timings::new(cli.timings, start);
    let result = core::run_file(&cli.source, &cli.output, &cli.environment, &mut timings);
    timings.report();
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}
