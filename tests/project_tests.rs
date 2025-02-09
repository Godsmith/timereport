use rstest::*;
mod utils;
use tempfile::TempDir;
use utils::*;

#[rstest]
fn report_on_project(temp_dir: TempDir) {
    run("add myproject", &temp_dir);
    let output = run("project myproject 8", &temp_dir); // TODO: add to run instead

    assert!(output.contains("08:00"));
}

#[rstest]
fn report_on_unknown_project_prints_error(temp_dir: TempDir) {
    let output = run("project myproject 8", &temp_dir); // TODO: add to run instead

    assert!(output.contains("Unknown project 'myproject'"))
}

#[rstest]
fn show_default_project_name(temp_dir: TempDir) {
    let output = run("", &temp_dir); // TODO: add to run instead

    assert!(output.contains("Default project"));
}

#[rstest]
fn show_number_before_project_name(temp_dir: TempDir) {
    run("add myproject", &temp_dir);
    let output = run("", &temp_dir); // TODO: add to run instead

    assert!(output.contains("2. myproject"));
}

#[rstest]
fn report_on_number(temp_dir: TempDir) {
    run("add myproject", &temp_dir);
    let output = run("project 2 8", &temp_dir); // TODO: add to run instead

    assert!(output.contains("08:00"));
}

#[rstest]
fn report_on_0_gives_error(temp_dir: TempDir) {
    run("add myproject", &temp_dir);
    let output = run("project 0 8", &temp_dir); // TODO: add to run instead

    assert!(output.contains("No project with index 0"));
}

#[rstest]
fn report_on_1_gives_error(temp_dir: TempDir) {
    run("add myproject", &temp_dir);
    let output = run("project 1 8", &temp_dir); // TODO: add to run instead

    assert!(output.contains("Cannot report time on default project"));
}

#[rstest]
fn report_on_999_gives_error(temp_dir: TempDir) {
    run("add myproject", &temp_dir);
    let output = run("project 999 8", &temp_dir); // TODO: add to run instead

    assert!(output.contains("No project with index 999"));
}

#[rstest]
fn report_on_two_projects(temp_dir: TempDir) {
    run("add p1", &temp_dir);
    run("add p2", &temp_dir);
    run("project p1 1", &temp_dir);
    let output = run("project p2 2", &temp_dir);

    assert!(output.contains("1:00"));
    assert!(output.contains("2:00"));
}

#[rstest]
fn show_default_project_time(temp_dir: TempDir) {
    let output = run("start 8:00 stop 16:00 lunch 45m", &temp_dir);

    assert!(output.contains("7:15"));
}

#[rstest]
fn subtract_project_time_from_default_project_time(temp_dir: TempDir) {
    run("add p1", &temp_dir);
    run("project p1 1", &temp_dir);
    let output = run("start 8:00 stop 16:00 lunch 45m", &temp_dir);

    assert!(output.contains("6:15"));
}

// TODO: add project with multiple words
