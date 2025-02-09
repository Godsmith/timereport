use rstest::*;
use tempfile::{tempdir, TempDir};
use timereport::main;

#[fixture]
pub fn temp_dir() -> TempDir {
    tempdir().expect("Could not create tempdir")
    //dir.path().join("timereport.json")
}

pub fn run(s: &str, temp_dir: &TempDir) -> String {
    let mut args: Vec<String> = s.split_whitespace().map(|s| s.to_string()).collect();
    // This might not be a good idea, this means that we cannot test for the weekend
    // NOT showing. Better to only show the weekend when data on the weekend perhaps?
    // or when editing a weekend? or when it is a weekend today?
    args.push("--weekend".to_string());
    main(args, &temp_dir.path().join("timereport.json").as_path())
}
