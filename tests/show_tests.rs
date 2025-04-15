use rstest::*;
use tempfile::TempDir;
use timereport::mockopen::open::FILE_CONTENT;
mod utils;
use chrono::prelude::*;
use chrono::{Duration, Local};
use utils::*;

#[rstest]
fn show_week(temp_dir: TempDir) {
    let output = run("show week", &temp_dir);

    let today = Local::now().format("%Y-%m-%d").to_string();
    assert!(output.contains(&today));
}

#[rstest]
fn show_last_week(temp_dir: TempDir) {
    let one_week_ago = Local::now() - Duration::try_weeks(1).expect("hardcoded int");
    let one_week_ago = one_week_ago.format("%Y-%m-%d").to_string();

    let output = run("show last week --weekend", &temp_dir);

    assert!(output.contains(&one_week_ago));
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
fn show_week_html_prints_copy_button(temp_dir: TempDir) {
    run("show week html", &temp_dir);
    FILE_CONTENT.with(|content| {
        let content = content.borrow();
        assert!(content.contains("<button onclick=\"copyToClipboard("));
        assert!(content.contains(")\">1. Default project</button>"));
    })
}

#[rstest]
fn show_week_html_prints_needed_flex(temp_dir: TempDir) {
    run("start 10 stop 17 lunch 0", &temp_dir);
    run("show week html", &temp_dir);
    FILE_CONTENT.with(|content| {
        let content = content.borrow();
        assert!(content.contains("0,75"));
    })
}

#[rstest]
fn show_unknown(temp_dir: TempDir) {
    let output = run("show foo", &temp_dir);

    assert!(output.contains("Unknown show command"));
    assert!(output.contains("foo"));
}

#[rstest]
fn show_month_first_day_of_month(temp_dir: TempDir) {
    let output = run("show january", &temp_dir);
    let current_year = Local::now().year();

    assert!(output.contains(format!("{}-01-01", current_year).as_str()));
}

#[rstest]
fn show_month_last_day_of_month(temp_dir: TempDir) {
    let output = run("show january", &temp_dir);
    let current_year = Local::now().year();

    assert!(output.contains(format!("{}-01-31", current_year).as_str()));
}
