mod cli;
mod core;

use std::process::ExitCode;

use crate::cli::Cli;

fn main() -> ExitCode {
    let cli = Cli::parse();
    match core::run_file(&cli.source, &cli.output, &cli.environment) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}
