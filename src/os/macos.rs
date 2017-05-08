extern crate libc;

use std;
use std::ffi::{CString, CStr, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use super::super::Error;

/// Find the mountpoint for the volume containing the (previously canonicalized) provided path.
///
/// This uses `libc::statfs`, and as a result is (partially) unsafe.
///
/// It presumes that you will be passing in fully-qualified canonical paths (to minimize allocation
/// overhead, because the mountpoint that gets returned is a slice of the path that was passed in).
#[cfg(target_os = "macos")]
pub fn find_mountpoint_pre_canonicalized(path: &Path) -> Result<&Path, Error> {
    let cstr = CString::new(path.as_os_str().as_bytes())?;

    let raw_mountpoint = unsafe {
        let mut fs_stat: libc::statfs = std::mem::uninitialized();
        if libc::statfs(cstr.as_ptr(), &mut fs_stat) != 0 {
            return Err(Error::from(std::io::Error::last_os_error()));
        } else {
            // CStr::from_ptr uses `libc::strlen` to delimit the slice and `transmute` to cast.
            CStr::from_ptr(fs_stat.f_mntonname.as_ptr())
        }
    };

    // from_bytes comes from OsStrExt -- it would be nice to figure out a more direct way to go
    // from CStr to OsStr
    let mountpoint = OsStr::from_bytes(raw_mountpoint.to_bytes());

    // I may remove these later, but for now I want to verify that these invariants hold in the
    // wild.
    assert!(path.starts_with(mountpoint));
    assert!(mountpoint.len() > 0);

    Ok(&Path::new(mountpoint))
}

/// Find the mountpoint for the volume containing the provided path.
///
/// Canonicalizes the path before calling `find_mountpoint_pre_canonicalized`. Because
/// canonicalization produces a PathBuf, lifetime management requires returning an owned path,
/// hence returns a PathBuf instead of a reference to a Path.
#[cfg(target_os = "macos")]
pub fn find_mountpoint(path: &Path) -> Result<PathBuf, Error> {
    let canonicalized = path.canonicalize()?;
    let found = find_mountpoint_pre_canonicalized(canonicalized.as_path())?;
    Ok(found.to_path_buf())
}
