mod tests {
    use rstest::rstest;
    use timereport::parse_date;
    #[rstest]
    fn parse_date_error() {
        let output = parse_date("2024-01-32");
        assert_eq!(
            output,
            Result::Err(
                "Could not parse date string '2024-01-32'. Error: 'input contains invalid characters'"
                    .to_string()
            )
        );
    }
}
