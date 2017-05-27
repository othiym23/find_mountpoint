use super::super::Error;

use std::path::{Component, Path, PathBuf};

/// Find the prefix (drive letter or UNC path / sharepoint) for the (already canonicalized)
/// provided paths.
///
/// This version relies on Windows-only `PrefixComponents` but is safe. Which is good, because it's
/// completely untested.
///
/// It presumes that you will be passing in fully-qualified canonical paths.
pub fn find_mountpoint_pre_canonicalized(path: &Path) -> Result<&Path, Error> {
    for component in path.components() {
        if let Component::Prefix(prefix) = component {
            return Ok(Path::new(prefix.as_os_str()));
        }
    }

    Err(Error::from("Couldn't find prefix on path.".to_owned()))
}

/// Find the prefix (drive letter or UNC path / sharepoint) for the provided paths.
///
/// Canonicalizes the path before calling `find_mountpoint_pre_canonicalized`. Because
/// canonicalization produces a `PathBuf`, lifetime management requires returning an owned path,
/// hence returns a `PathBuf` instead of a reference to a Path.
pub fn find_mountpoint(path: &Path) -> Result<PathBuf, Error> {
    let canonicalized = path.canonicalize()?;
    let found = find_mountpoint_pre_canonicalized(canonicalized.as_path())?;
    Ok(found.to_path_buf())
}
