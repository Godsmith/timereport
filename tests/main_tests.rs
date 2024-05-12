use chrono::Local;
use timereport::main;

fn run(s: &str) -> String {
    main(s.split_whitespace().map(|s| s.to_string()).collect())
}

#[test]
fn no_arguments_give_error() {
    let output = main(vec![]);
    assert_eq!(output, "start is None");
}

#[test]
fn start_2024_06_26_at_8_30_prints_table() {
    let output = run("2024-04-26 start 8:30");
    let expected = r#"+-------+------------+------------+------------+------------+------------+
|       | 2024-04-22 | 2024-04-23 | 2024-04-24 | 2024-04-25 | 2024-04-26 |
+-------+------------+------------+------------+------------+------------+
| start |            |            |            |            | 08:30      |
+-------+------------+------------+------------+------------+------------+"#;
    assert_eq!(output, expected);
}

#[test]
fn start_8_30_weekend_prints_current_date_and_8_30() {
    let today = Local::now().format("%Y-%m-%d").to_string();

    let output = run("start 8:30 --weekend");

    assert!(output.contains(&today));
    assert!(output.contains("08:30"));
}
