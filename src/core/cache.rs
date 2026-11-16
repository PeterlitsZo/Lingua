use std::{
    fs::{self, Metadata, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::UNIX_EPOCH,
};

use super::Program;

const MAGIC: &[u8; 8] = b"LINGUA\0\0";
// Bump for incompatible encoding, instruction, or compiler semantics changes.
const VERSION: u32 = 1;
static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct SourceStamp {
    seconds: u64,
    nanos: u32,
    size: u64,
}

impl SourceStamp {
    pub(super) fn new(metadata: &Metadata) -> Option<Self> {
        let modified = metadata.modified().ok()?.duration_since(UNIX_EPOCH).ok()?;
        metadata.is_file().then_some(Self {
            seconds: modified.as_secs(),
            nanos: modified.subsec_nanos(),
            size: metadata.len(),
        })
    }

    fn header(self) -> Vec<u8> {
        let mut bytes = MAGIC.to_vec();
        bytes.extend_from_slice(&VERSION.to_le_bytes());
        bytes.extend_from_slice(&self.seconds.to_le_bytes());
        bytes.extend_from_slice(&self.nanos.to_le_bytes());
        bytes.extend_from_slice(&self.size.to_le_bytes());
        bytes
    }
}

pub(super) struct SourceCache {
    path: PathBuf,
    stamp: SourceStamp,
}

impl SourceCache {
    pub(super) fn new(source: &Path, target: &Path, stamp: SourceStamp) -> Option<Self> {
        let parent = source
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        let parent = fs::canonicalize(parent).ok()?;
        let directory = parent.join("__lingua_cache__");
        let mut name = source.file_name()?.to_os_string();
        name.push(".lbc");
        let path = directory.join(name);
        // Resolve existing ancestors so aliases and not-yet-created paths compare equally.
        let source = resolve_path(source)?;
        let target = resolve_path(target)?;
        if source.starts_with(&directory) || target.starts_with(&directory) {
            return None;
        }
        Some(Self { path, stamp })
    }

    pub(super) fn load(&self) -> Option<Program> {
        if !self.safe_directory() || !fs::symlink_metadata(&self.path).ok()?.file_type().is_file() {
            return None;
        }
        let bytes = fs::read(&self.path).ok()?;
        let header = self.stamp.header();
        if !bytes.starts_with(&header) {
            return None;
        }
        Program::decode(&bytes[header.len()..])
    }

    pub(super) fn store(&self, program: &Program) {
        let _ = self.write(program);
    }

    fn safe_directory(&self) -> bool {
        fs::symlink_metadata(self.path.parent().expect("cache parent"))
            .is_ok_and(|metadata| metadata.file_type().is_dir())
    }

    fn write(&self, program: &Program) -> Option<()> {
        let payload = program.encode()?;
        let directory = self.path.parent()?;
        match fs::create_dir(directory) {
            Ok(()) => {}
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {}
            Err(_) => return None,
        }
        if !self.safe_directory() {
            return None;
        }
        match fs::symlink_metadata(&self.path) {
            Ok(metadata) if !metadata.file_type().is_file() => return None,
            Ok(_) => {}
            Err(err) if err.kind() == io::ErrorKind::NotFound => {}
            Err(_) => return None,
        }
        let temporary = directory.join(format!(
            ".{}.{}.tmp",
            std::process::id(),
            NEXT_TEMP.fetch_add(1, Ordering::Relaxed)
        ));
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .ok()?;
        let result = (|| {
            file.write_all(&self.stamp.header())?;
            file.write_all(&payload)?;
            file.flush()
        })();
        drop(file);
        let published =
            result.is_ok() && self.safe_directory() && fs::rename(&temporary, &self.path).is_ok();
        if !published {
            let _ = fs::remove_file(&temporary);
        }
        published.then_some(())
    }
}

fn resolve_path(path: &Path) -> Option<PathBuf> {
    if let Ok(path) = fs::canonicalize(path) {
        return Some(path);
    }
    let name = path.file_name()?;
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    Some(resolve_path(parent)?.join(name))
}
