pub mod error;

pub use error::{Error, ErrorKind};

use std::fs;
use std::io;
use std::path::Path;

pub fn copy_contents(source: &Path, target: &Path) -> Result<(), Error> {
    copy_contents_inner(source, target).map_err(|err| {
        err.with_context("copying source bytes to output")
            .with_metadata("source", source.display().to_string())
            .with_metadata("output", target.display().to_string())
    })
}

fn copy_contents_inner(source: &Path, target: &Path) -> Result<(), Error> {
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
    fs::write(target, contents)
        .map_err(|err| Error::new(ErrorKind::IO, "Cannot write target").with_source(err))
}
