#![feature(test)]
extern crate test;


#[cfg(test)]
mod tests {
    extern crate volume_prefix;

    use self::volume_prefix::*;
    use std::path::Path;
    use test::Bencher;

    #[bench]
    fn bench_pre_canonicalized(benchr: &mut Bencher) {
        let sample = Path::new("./Cargo.toml").canonicalize().unwrap();
        benchr.iter(|| find_mountpoint_pre_canonicalized(sample.as_path()).unwrap())
    }

    #[bench]
    fn bench_canonicalize(benchr: &mut Bencher) {
        let sample = Path::new("./Cargo.toml");
        benchr.iter(|| find_mountpoint(sample).unwrap())
    }
}
