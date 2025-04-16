use chrono::{Duration, Local, NaiveDate};
use regex::Regex;
use rstest::*;
use std::fs::File;
use std::io::Read;
use tempfile::TempDir;
mod utils;
use utils::*;

fn config_contents(temp_dir: &TempDir) -> String {
    let mut contents = String::new();
    let path_buf = temp_dir.path().join("timereport.json");
    let path = path_buf.as_path();
    let mut file = File::open(path).expect("");
    file.read_to_string(&mut contents).expect("");
    contents
}

#[rstest]
fn no_arguments_prints_current_date(temp_dir: TempDir) {
    let today = Local::now().format("%Y-%m-%d").to_string();

    let output = run("", &temp_dir);

    assert!(output.contains(&today));
}

#[rstest]
fn no_argument_does_not_affect_config_file(temp_dir: TempDir) {
    run("", &temp_dir); // To create a config file
    let config_before = config_contents(&temp_dir);
    run("", &temp_dir);
    let config_after = config_contents(&temp_dir);
    assert!(config_before == config_after)
}

#[rstest]
fn partially_correct_command_does_not_affect_config_file(temp_dir: TempDir) {
    run("", &temp_dir); // To create a config file
    let config_before = config_contents(&temp_dir);
    run("start 8 blargh", &temp_dir);
    let config_after = config_contents(&temp_dir);
    assert!(config_before == config_after)
}

#[rstest]
fn start(temp_dir: TempDir) {
    let re = Regex::new(r"start.*08:30").unwrap();
    let output = run("2024-06-26 start 8:30", &temp_dir);

    assert!(re.is_match(&output))
}

#[rstest]
fn stop(temp_dir: TempDir) {
    let re = Regex::new(r"stop.*08:30").unwrap();
    let output = run("2024-06-26 stop 8:30", &temp_dir);

    assert!(re.is_match(&output))
}

#[rstest]
fn start_8_30_weekend_prints_current_date_and_8_30(temp_dir: TempDir) {
    let today = Local::now().format("%Y-%m-%d").to_string();

    let output = run("start 8:30", &temp_dir);

    assert!(output.contains(&today));
    assert!(output.contains("08:30"));
}

#[rstest]
fn bold_formatting_for_changed_cell(temp_dir: TempDir) {
    let output = run("2024-06-26 start 8:30", &temp_dir);
    assert!(output.contains("\x1b[1m08:30"))
}

#[rstest]
fn report_only_hour(temp_dir: TempDir) {
    let output = run("start 8", &temp_dir);

    assert!(output.contains("08:00"));
}

#[rstest]
fn lunch_hours_and_minutes(temp_dir: TempDir) {
    let today = Local::now().format("%Y-%m-%d").to_string();

    let output = run("lunch 1:75", &temp_dir);

    assert!(output.contains(&today));
    assert!(output.contains("2:15"));
}

#[rstest]
fn report_weekday(temp_dir: TempDir) {
    let output = run("monday start 8:00", &temp_dir);

    assert!(output.contains("8:00"));
}

#[rstest]
fn report_yesterday(temp_dir: TempDir) {
    let output = run_mock_date(
        "yesterday start 8:00",
        &temp_dir,
        // Need to mock date to ensure that the time
        // is always displayed in the output
        NaiveDate::from_ymd_opt(2025, 4, 15).expect(""),
    );

    assert!(output.contains("8:00"));
}

#[rstest]
fn report_day_last_week(temp_dir: TempDir) {
    let one_week_ago = Local::now() - Duration::try_weeks(1).expect("hardcoded int");
    let one_week_ago = one_week_ago.format("%Y-%m-%d").to_string();
    let output = run("last saturday start 8:00", &temp_dir);

    assert!(output.contains(&one_week_ago));
    assert!(output.contains("8:00"));
}

#[rstest]
fn report_multiple_days(temp_dir: TempDir) {
    let output = run("monday tuesday start 8", &temp_dir);
    assert!(output.matches("8:00").count() == 2);
}

#[rstest]
fn subsequent_edits_of_the_same_day(temp_dir: TempDir) {
    run("start 7:00", &temp_dir);
    let output = run("stop 15:00", &temp_dir);
    assert!(output.contains("7:00"));
    assert!(output.contains("15:00"));
}

#[rstest]
fn lunch_is_correctly_parsed_from_file(temp_dir: TempDir) {
    run("lunch 45m", &temp_dir);
    let output = run("show week", &temp_dir);
    assert!(output.contains("45"));
}

#[rstest]
fn lunch_minutes_and_m(temp_dir: TempDir) {
    let today = Local::now().format("%Y-%m-%d").to_string();

    let output = run("lunch 15m", &temp_dir);

    assert!(output.contains(&today));
    assert!(output.contains("0:15"));
}

#[rstest]
fn extra_argument(temp_dir: TempDir) {
    let output = run("foo", &temp_dir);

    assert!(output.contains("Unknown or extra argument"));
    assert!(output.contains("foo"));
}

#[rstest]
fn undo(temp_dir: TempDir) {
    let output = run("start 8", &temp_dir);
    assert!(output.contains("8:00"));

    let output = run("undo", &temp_dir);

    assert!(!output.contains("8:00"));
}

#[rstest]
fn nothing_to_undo(temp_dir: TempDir) {
    let output = run("undo", &temp_dir);

    assert!(output.contains("Nothing to undo"));
}

#[rstest]
fn redo(temp_dir: TempDir) {
    run("start 8", &temp_dir);
    run("undo", &temp_dir);
    let output = run("redo", &temp_dir);

    assert!(output.contains("8:00"));
}

#[rstest]
fn nothing_to_redo(temp_dir: TempDir) {
    let output = run("redo", &temp_dir);

    assert!(output.contains("Nothing to redo"));
}

#[rstest]
fn adding_day_clears_undone(temp_dir: TempDir) {
    run("start 8", &temp_dir);
    run("undo", &temp_dir);
    run("start 8", &temp_dir);
    let output = run("redo", &temp_dir);

    assert!(output.contains("Nothing to redo"));
}

#[rstest]
fn positive_flex(temp_dir: TempDir) {
    let output = run("start 8 stop 17 lunch 45m", &temp_dir);

    assert!(output.contains("00:30"));
}

#[rstest]
fn negative_flex(temp_dir: TempDir) {
    let output = run("start 8 stop 15 lunch 45m", &temp_dir);

    assert!(output.contains("-01:30"));
}

#[rstest]
fn version(temp_dir: TempDir) {
    let output = run("--version", &temp_dir);

    assert!(output.contains("."));
}

#[rstest]
fn help(temp_dir: TempDir) {
    let output = run("--help", &temp_dir);

    assert!(output.contains("Usage"));
}
