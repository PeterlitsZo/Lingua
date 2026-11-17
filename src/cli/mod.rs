mod binding;

use std::{ffi::OsStr, path::PathBuf};

use clap::{CommandFactory, Parser, error::ErrorKind};

use self::binding::VariableAssignment;
use crate::core::Environment;

#[derive(Parser)]
#[command(version, about = "Process a Lingua file")]
struct Args {
    /// Source file
    #[arg(value_name = "file.lingua")]
    source: PathBuf,
    /// Output file
    #[arg(short = 'o', value_name = "OUTPUT-FILE")]
    output: Option<PathBuf>,
    /// Set a variable
    #[arg(short = 'v', value_name = "NAME=VALUE")]
    variables: Vec<VariableAssignment>,
    /// Print stage timings and total elapsed time to stderr
    #[arg(long)]
    timings: bool,
}

pub struct Cli {
    pub source: PathBuf,
    pub output: PathBuf,
    pub environment: Environment,
    pub timings: bool,
}

impl Cli {
    pub fn parse() -> Self {
        let Args {
            source,
            output,
            variables,
            timings,
        } = Args::parse();
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
        Self {
            source,
            output,
            environment,
            timings,
        }
    }
}
