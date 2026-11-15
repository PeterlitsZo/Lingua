mod ast;
mod bytecode;
pub mod error;

use std::{fs, io, path::Path};

use self::{
    ast::Ast,
    bytecode::{BytecodeCompiler, BytecodeRunner},
};

pub use self::error::{Error, ErrorKind};

pub fn run_file(source: &Path, target: &Path) -> Result<(), Error> {
    run_file_inner(source, target).map_err(|err| {
        err.with_context("compiling and executing source file")
            .with_metadata("source", source.display().to_string())
            .with_metadata("output", target.display().to_string())
    })
}

fn run_file_inner(source: &Path, target: &Path) -> Result<(), Error> {
    let metadata = fs::metadata(source)
        .map_err(|err| Error::new(ErrorKind::IO, "Cannot inspect source").with_source(err))?;
    if !metadata.is_file() {
        return Err(Error::new(
            ErrorKind::InvalidSource,
            "Source is not a regular file",
        ));
    }

    match fs::symlink_metadata(target) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            return Err(Error::new(
                ErrorKind::InvalidTarget,
                "Refusing symbolic-link target",
            ));
        }
        Ok(metadata) if !metadata.is_file() => {
            return Err(Error::new(
                ErrorKind::InvalidTarget,
                "Target is not a regular file",
            ));
        }
        Ok(_) => {}
        Err(err) if err.kind() == io::ErrorKind::NotFound => {}
        Err(err) => {
            return Err(Error::new(ErrorKind::IO, "Cannot inspect target").with_source(err));
        }
    }

    let contents = fs::read(source)
        .map_err(|err| Error::new(ErrorKind::IO, "Cannot read source").with_source(err))?;
    let source_text = String::from_utf8(contents).map_err(|err| {
        Error::new(ErrorKind::InvalidSource, "Source is not valid UTF-8").with_source(err)
    })?;
    let ast = Ast::parse(source_text);
    let bytecode = BytecodeCompiler::new(ast).compile();
    let output = BytecodeRunner::new(&bytecode).run()?;
    fs::write(target, output)
        .map_err(|err| Error::new(ErrorKind::IO, "Cannot write target").with_source(err))
}
