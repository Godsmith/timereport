use chrono::Local;
use rstest::*;
use tempfile::{tempdir, TempDir};
use timereport::main;
use timereport::mockopen::open::FILE_CONTENT;

#[fixture]
fn temp_dir() -> TempDir {
    tempdir().expect("Could not create tempdir")
    //dir.path().join("timereport.json")
}

fn run(s: &str, temp_dir: &TempDir) -> String {
    main(
        s.split_whitespace().map(|s| s.to_string()).collect(),
        &temp_dir.path().join("timereport.json").as_path(),
    )
}

#[rstest]
fn no_arguments_prints_current_date(temp_dir: TempDir) {
    let today = Local::now().format("%Y-%m-%d").to_string();

    let output = run("--weekend", &temp_dir);

    assert!(output.contains(&today));
}

#[rstest]
fn start_2024_06_26_at_8_30_prints_table(temp_dir: TempDir) {
    let output = run("2024-04-26 start 8:30", &temp_dir);
    let expected = r#"+-------+------------+------------+------------+------------+------------+
|       | 2024-04-22 | 2024-04-23 | 2024-04-24 | 2024-04-25 | 2024-04-26 |
+-------+------------+------------+------------+------------+------------+
|       | Monday     | Tuesday    | Wednesday  | Thursday   | Friday     |
+-------+------------+------------+------------+------------+------------+
| start |            |            |            |            | 08:30      |
+-------+------------+------------+------------+------------+------------+
| stop  |            |            |            |            |            |
+-------+------------+------------+------------+------------+------------+
| lunch |            |            |            |            |            |
+-------+------------+------------+------------+------------+------------+"#;
    assert_eq!(output, expected);
}

#[rstest]
fn stop(temp_dir: TempDir) {
    let output = run("2024-04-26 stop 8:30", &temp_dir);
    let expected = r#"+-------+------------+------------+------------+------------+------------+
|       | 2024-04-22 | 2024-04-23 | 2024-04-24 | 2024-04-25 | 2024-04-26 |
+-------+------------+------------+------------+------------+------------+
|       | Monday     | Tuesday    | Wednesday  | Thursday   | Friday     |
+-------+------------+------------+------------+------------+------------+
| start |            |            |            |            |            |
+-------+------------+------------+------------+------------+------------+
| stop  |            |            |            |            | 08:30      |
+-------+------------+------------+------------+------------+------------+
| lunch |            |            |            |            |            |
+-------+------------+------------+------------+------------+------------+"#;
    assert_eq!(output, expected);
}

#[rstest]
fn start_8_30_weekend_prints_current_date_and_8_30(temp_dir: TempDir) {
    let today = Local::now().format("%Y-%m-%d").to_string();

    let output = run("start 8:30 --weekend", &temp_dir);

    assert!(output.contains(&today));
    assert!(output.contains("08:30"));
}

#[rstest]
fn report_only_hour(temp_dir: TempDir) {
    let output = run("start 8 --weekend", &temp_dir);

    assert!(output.contains("08:00"));
}

#[rstest]
fn lunch_hours_and_minutes(temp_dir: TempDir) {
    let today = Local::now().format("%Y-%m-%d").to_string();

    let output = run("lunch 1:75 --weekend", &temp_dir);

    assert!(output.contains(&today));
    assert!(output.contains("2:15"));
}

#[rstest]
fn report_weekday(temp_dir: TempDir) {
    let output = run("monday start 8:00", &temp_dir);

    assert!(output.contains("8:00"));
}

#[rstest]
fn subsequent_edits_of_the_same_day(temp_dir: TempDir) {
    run("start 7:00 --weekend", &temp_dir);
    let output = run("stop 15:00 --weekend", &temp_dir);
    assert!(output.contains("7:00"));
    assert!(output.contains("15:00"));
}

#[rstest]
fn lunch_is_correctly_parsed_from_file(temp_dir: TempDir) {
    run("lunch 45m --weekend", &temp_dir);
    let output = run("show week --weekend", &temp_dir);
    assert!(output.contains("45"));
}

#[rstest]
fn lunch_minutes_and_m(temp_dir: TempDir) {
    let today = Local::now().format("%Y-%m-%d").to_string();

    let output = run("lunch 15m --weekend", &temp_dir);

    assert!(output.contains(&today));
    assert!(output.contains("0:15"));
}

#[rstest]
fn show_week(temp_dir: TempDir) {
    let output = run("show week --weekend", &temp_dir);

    let today = Local::now().format("%Y-%m-%d").to_string();
    assert!(output.contains(&today));
}

#[rstest]
fn show_week_html_prints_table(temp_dir: TempDir) {
    run("show week html", &temp_dir);
    FILE_CONTENT.with(|content| {
        let content = content.borrow();
        assert!(content.contains("<table>"));
        assert!(content.contains("lunch"));
    })
}

#[rstest]
fn show_unknown(temp_dir: TempDir) {
    let output = run("show foo", &temp_dir);

    assert!(output.contains("Unknown show command"));
    assert!(output.contains("foo"));
}

#[rstest]
fn extra_argument(temp_dir: TempDir) {
    let output = run("foo", &temp_dir);

    assert!(output.contains("Unknown or extra argument"));
    assert!(output.contains("foo"));
}
