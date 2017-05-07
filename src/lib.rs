extern crate libc;

use std::ffi::{CString, CStr, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

use libc::statfs;

#[derive(Debug)]
pub enum Error {
    Ffi(std::ffi::NulError),
    Io(std::io::Error),
    S(String),
}

impl From<std::ffi::NulError> for Error {
    fn from(err: std::ffi::NulError) -> Error {
        Error::Ffi(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::S(err)
    }
}

/// Given a path, find the prefix for the current path that is the mount point prefix.  This uses
/// `statfs` from `libc`, and presumse that you will be passing in fully-qualified canonical paths
/// (to minimize allocation overhead, because the mountpoint that gets returned is a slice of the
/// path that was passed in).
pub fn get_mount_prefix(path: &Path) -> Result<&Path, Error> {
    let cstr = CString::new(path.to_str().unwrap())?;

    let raw_mountpoint = unsafe {
        let mut fs_stat: statfs = std::mem::uninitialized();
        if statfs(cstr.as_ptr(), &mut fs_stat) != 0 {
            return Err(Error::from(std::io::Error::last_os_error()));
        } else {
            // CStr::from_ptr seeks NUL to delimit the slice
            CStr::from_ptr(fs_stat.f_mntonname.as_ptr())
        }
    };

    // from_bytes comes from OsStrExt -- it would be nice to figure out a more direct way to go
    // from CStr to OsStr
    let mountpoint = OsStr::from_bytes(raw_mountpoint.to_bytes());
    let len = mountpoint.len();
    assert!(path.starts_with(mountpoint));
    assert!(len > 0);

    let view = path.as_os_str().as_bytes();
    Ok(&Path::new(OsStr::from_bytes(&view[..len])))
}

#[cfg(test)]
mod tests {
    use super::get_mount_prefix;
    use std::path::Path;

    #[test]
    fn integration_basic() {
        let sample = Path::new("./Cargo.toml").canonicalize().unwrap();
        let result = get_mount_prefix(sample.as_path());
        assert_eq!(result.unwrap().to_str().unwrap(), "/");
    }

    #[test]
    fn integration_another_fs() {
        let sample = Path::new("/Volumes/energeia/CONFIG.SYS");
        let result = get_mount_prefix(sample);
        assert_eq!(result.unwrap().to_str().unwrap(), "/Volumes/energeia");
    }

    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn nonexistent_path() {
        let sample = Path::new("/Volumes/NOxSUCHxMOUNT/prj/Cargo.toml");
        let result = get_mount_prefix(sample).unwrap();
        assert_eq!(result.to_str().unwrap(), "/");
    }
}
