mod os;

#[cfg(all(unix, not(target_os = "macos")))]
pub use os::unix::*;
#[cfg(target_os = "macos")]
pub use os::macos::*;
#[cfg(windows)]
pub use os::windows::*;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn integration_basic() {
        let sample = Path::new("./Cargo.toml").canonicalize().unwrap();
        let result = find_mountpoint_pre_canonicalized(sample.as_path());
        assert_eq!(result.unwrap().to_str().unwrap(), "/");
    }

    #[test]
    fn integration_basic_without_canonicalization() {
        let sample = Path::new("./Cargo.toml");
        let result = find_mountpoint(sample);
        assert_eq!(result.unwrap().to_str().unwrap(), "/");
    }

    // only run this if you have a Boot Camp volume named energeia on your system.
    #[test]
    #[ignore]
    fn integration_another_fs() {
        let sample = Path::new("/Volumes/energeia/CONFIG.SYS");
        let result = find_mountpoint_pre_canonicalized(sample);
        assert_eq!(result.unwrap().to_str().unwrap(), "/Volumes/energeia");
    }

    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn nonexistent_path() {
        let sample = Path::new("/Volumes/NOxSUCHxMOUNT/prj/Cargo.toml");
        find_mountpoint_pre_canonicalized(sample).unwrap();
    }

    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn nonexistent_path_without_canonicalization() {
        let sample = Path::new("../vegan/porkchop/sandwiches");
        find_mountpoint(sample).unwrap();
    }
}
