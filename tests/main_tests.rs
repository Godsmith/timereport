use timereport::main;

#[test]
fn no_arguments_give_error() {
    let output = main(vec![]);
    assert_eq!(output, "start is None");
}

#[test]
fn came_8_30_prints_table() {
    let output = main(vec![String::from("came"), String::from("8:30")]);
    let expected = r#"+------+------------+------------+------------+------------+------------+
|      | 2024-04-22 | 2024-04-23 | 2024-04-24 | 2024-04-25 | 2024-04-26 |
+------+------------+------------+------------+------------+------------+
| came |            |            |            |            | 08:30      |
+------+------------+------------+------------+------------+------------+"#;
    assert_eq!(output, expected);
}
