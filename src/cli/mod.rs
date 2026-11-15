use clap::{CommandFactory, Parser, error::ErrorKind};
use std::ffi::OsStr;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    version,
    about = "Copy a file's bytes to the path without its final .lingua suffix"
)]
struct Args {
    /// Source file to copy
    #[arg(value_name = "file.lingua")]
    source: PathBuf,
}

pub struct Cli {
    pub source: PathBuf,
    pub output: PathBuf,
}

impl Cli {
    pub fn parse() -> Self {
        let Args { source } = Args::parse();
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
        let stem = source
            .file_stem()
            .filter(|stem| !stem.is_empty())
            .expect("a path with a .lingua extension has a nonempty stem");
        let output = source.with_file_name(stem);
        Self { source, output }
    }
}
