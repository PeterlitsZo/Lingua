mod binding;

use std::{ffi::OsStr, path::PathBuf};

use clap::{CommandFactory, Parser, Subcommand, error::ErrorKind};

use self::binding::VariableAssignment;
use crate::core::Environment;

#[derive(Parser)]
#[command(
    version,
    about = "Process a Lingua file or inspect bytecode",
    args_conflicts_with_subcommands = true,
    arg_required_else_help = true
)]
struct Args {
    /// Source file
    #[arg(value_name = "file.lingua")]
    source: Option<PathBuf>,
    /// Output file
    #[arg(short = 'o', value_name = "OUTPUT-FILE")]
    output: Option<PathBuf>,
    /// Set a variable
    #[arg(short = 'v', value_name = "NAME=VALUE")]
    variables: Vec<VariableAssignment>,
    /// Print stage timings and total elapsed time to stderr
    #[arg(long)]
    timings: bool,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Display an LBC file's header, constants, and instructions
    Disasm {
        #[arg(value_name = "file.lbc")]
        path: PathBuf,
    },
}

pub enum Cli {
    Run {
        source: PathBuf,
        output: PathBuf,
        environment: Environment,
        timings: bool,
    },
    Disasm {
        path: PathBuf,
    },
}

impl Cli {
    pub fn parse() -> Self {
        let Args {
            source,
            output,
            variables,
            timings,
            command,
        } = Args::parse();
        if let Some(Command::Disasm { path }) = command {
            return Self::Disasm { path };
        }
        let source = source.unwrap_or_else(|| {
            Args::command()
                .error(
                    ErrorKind::MissingRequiredArgument,
                    "A source file is required",
                )
                .exit()
        });
        if source.extension() != Some(OsStr::new("lingua")) {
            Args::command()
                .error(
                    ErrorKind::ValueValidation,
                    format!(
                        "Invalid source '{}': expected a nonempty filename followed by .lingua.",
                        source.display()
                    ),
                )
                .exit();
        }
        let output = output.unwrap_or_else(|| {
            let stem = source
                .file_stem()
                .filter(|stem| !stem.is_empty())
                .expect("a path with a .lingua extension has a nonempty stem");
            source.with_file_name(stem)
        });
        let mut environment = Environment::default();
        for variable in variables {
            environment.insert(variable.name, variable.value);
        }
        Self::Run {
            source,
            output,
            environment,
            timings,
        }
    }
}
