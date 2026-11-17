use std::{fs, io, path::Path};

use super::{
    Environment, Error, ErrorKind, Program,
    cache::{SourceCache, SourceStamp},
    timing::Timings,
};

pub(crate) fn run_file(
    source: &Path,
    target: &Path,
    environment: &Environment,
    timings: &mut Timings,
) -> Result<(), Error> {
    run_file_inner(source, target, environment, timings).map_err(|err| {
        err.with_context("processing source file")
            .with_metadata("source", source.display().to_string())
            .with_metadata("output", target.display().to_string())
    })
}

fn run_file_inner(
    source: &Path,
    target: &Path,
    environment: &Environment,
    timings: &mut Timings,
) -> Result<(), Error> {
    let metadata = timings.measure("Path validation", || validate_paths(source, target))?;
    let cache = timings.measure("Cache lookup", || {
        let stamp = SourceStamp::new(&metadata);
        stamp.and_then(|stamp| SourceCache::new(source, target, stamp))
    });
    let cached = cache.as_ref().and_then(|cache| cache.load(timings));
    timings.cache_status = if cached.is_some() {
        "hit"
    } else if cache.is_some() {
        "miss"
    } else {
        "unavailable"
    };
    let program = match cached {
        Some(program) => program,
        None => {
            let (source_text, after_read) = timings.measure("Source read", || {
                let contents = fs::read(source).map_err(|err| {
                    Error::new(ErrorKind::IO, "Cannot read source").with_source(err)
                })?;
                let after_read = fs::metadata(source)
                    .ok()
                    .and_then(|metadata| SourceStamp::new(&metadata));
                let source_text = String::from_utf8(contents).map_err(|err| {
                    Error::new(ErrorKind::InvalidSource, "Source is not valid UTF-8")
                        .with_source(err)
                })?;
                Ok::<_, Error>((source_text, after_read))
            })?;
            let program = Program::compile(&source_text, timings)?;
            if SourceStamp::new(&metadata) == after_read
                && let Some(cache) = &cache
            {
                timings.measure("Cache write", || cache.store(&program));
            }
            program
        }
    };
    let output = timings.measure("Execute", || {
        program.run(environment).map(|value| value.into_text())
    })?;
    timings
        .measure("Output write", || fs::write(target, output))
        .map_err(|err| Error::new(ErrorKind::IO, "Cannot write target").with_source(err))
}

fn validate_paths(source: &Path, target: &Path) -> Result<fs::Metadata, Error> {
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

    Ok(metadata)
}
