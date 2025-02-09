use rstest::*;
use tempfile::{tempdir, TempDir};
use timereport::main;

#[fixture]
pub fn temp_dir() -> TempDir {
    tempdir().expect("Could not create tempdir")
    //dir.path().join("timereport.json")
}

pub fn run(s: &str, temp_dir: &TempDir) -> String {
    let args: Vec<String> = s.split_whitespace().map(|s| s.to_string()).collect();
    main(args, &temp_dir.path().join("timereport.json").as_path())
}
