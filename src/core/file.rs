use std::{fs, io, path::Path};

use super::{
    Environment, Error, ErrorKind, Program,
    cache::{SourceCache, SourceStamp},
};

pub(crate) fn run_file(
    source: &Path,
    target: &Path,
    environment: &Environment,
) -> Result<(), Error> {
    run_file_inner(source, target, environment).map_err(|err| {
        err.with_context("processing source file")
            .with_metadata("source", source.display().to_string())
            .with_metadata("output", target.display().to_string())
    })
}

fn run_file_inner(source: &Path, target: &Path, environment: &Environment) -> Result<(), Error> {
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

    let stamp = SourceStamp::new(&metadata);
    let cache = stamp.and_then(|stamp| SourceCache::new(source, target, stamp));
    let program = match cache.as_ref().and_then(SourceCache::load) {
        Some(program) => program,
        None => {
            let contents = fs::read(source)
                .map_err(|err| Error::new(ErrorKind::IO, "Cannot read source").with_source(err))?;
            let after_read = fs::metadata(source)
                .ok()
                .and_then(|metadata| SourceStamp::new(&metadata));
            let source_text = String::from_utf8(contents).map_err(|err| {
                Error::new(ErrorKind::InvalidSource, "Source is not valid UTF-8").with_source(err)
            })?;
            let program = Program::compile(&source_text)?;
            if stamp == after_read
                && let Some(cache) = &cache
            {
                cache.store(&program);
            }
            program
        }
    };
    let output = program.run(environment)?.into_text();

    fs::write(target, output)
        .map_err(|err| Error::new(ErrorKind::IO, "Cannot write target").with_source(err))
}
