mod cli;
mod core;

use std::{process::ExitCode, time::Instant};

use crate::cli::Cli;

fn main() -> ExitCode {
    let start = Instant::now();
    let result = match Cli::parse() {
        Cli::Run {
            source,
            output,
            environment,
            timings,
        } => {
            let mut timings = core::timing::Timings::new(timings, start);
            let result = core::run_file(&source, &output, &environment, &mut timings);
            timings.report();
            result
        }
        Cli::Disasm { path } => {
            core::disasm::disassemble_file(&path, &mut std::io::stdout().lock())
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}
