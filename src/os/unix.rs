#[cfg(unix)]
use super::super::Error;

#[cfg(unix)]
use std::path::{Path, PathBuf};

/// Find the mountpoint for the volume containing the (previously canonicalized) provided path.
///
/// This version relies on Unix-only extensions, but is safe.
///
/// It presumes that you will be passing in fully-qualified canonical paths.
#[cfg(unix)]
pub fn find_mountpoint_pre_canonicalized(path: &Path) -> Result<&Path, Error> {
    use std::fs::symlink_metadata;
    // needed to get dev for metadata (aliased as st_dev on Linux)
    use std::os::unix::fs::MetadataExt;

    let mut lstats = symlink_metadata(path)?;
    let start_dev = lstats.dev();

    let mut mountpoint = path;
    loop {
        let current = match mountpoint.parent() {
            Some(p) => p,
            None => return Ok(mountpoint),
        };
        lstats = symlink_metadata(current)?;
        if lstats.dev() != start_dev {
            break;
        }
        mountpoint = current;
    }

    // I may remove these later, but for now I want to verify that these invariants hold in the
    // wild.
    assert!(path.starts_with(mountpoint));
    assert!(mountpoint.as_os_str().len() > 0);

    Ok(mountpoint)
}

/// Find the mountpoint for the volume containing the provided path.
///
/// Canonicalizes the path before calling `find_mountpoint_pre_canonicalized`. Because
/// canonicalization produces a PathBuf, lifetime management requires returning an owned path,
/// hence returns a PathBuf instead of a reference to a Path.
#[cfg(unix)]
pub fn find_mountpoint(path: &Path) -> Result<PathBuf, Error> {
    let canonicalized = path.canonicalize()?;
    let found = find_mountpoint_pre_canonicalized(canonicalized.as_path())?;
    Ok(found.to_path_buf())
}
