use chrono::NaiveDate;

pub fn consume_bool(target: &str, args: Vec<String>) -> (bool, Vec<String>) {
    // Check if the target string exists in the vector
    let exists = args.iter().any(|s| s == target);

    // Create a new vector with the target string removed
    let filtered_args: Vec<String> = args.into_iter().filter(|s| *s != target).collect();

    // Return the tuple
    (exists, filtered_args)
}

pub fn consume_after_target(
    target: &str,
    args: Vec<String>,
) -> (Result<Option<String>, String>, Vec<String>) {
    args.iter().position(|s| s == target).map_or_else(
        || (Ok(None), args.to_vec()),
        |i| {
            if i >= args.len() - 1 {
                (Err(format!("No argument after {}", target)), args.to_vec())
            } else {
                let modified = args
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, s)| {
                        if idx != i && idx != i + 1 {
                            Some(s.clone())
                        } else {
                            None
                        }
                    })
                    .collect();

                (Ok(Some(args[i + 1].clone())), modified)
            }
        },
    )
}

pub fn consume_date(args: Vec<String>) -> (Option<NaiveDate>, Vec<String>) {
    // Find first date and its index
    let found = args.iter().enumerate().find_map(|(i, s)| {
        NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .ok()
            .map(|d| (i, d))
    });

    match found {
        Some((index, date)) => {
            // Create new vector without the date string
            let modified = args
                .into_iter()
                .enumerate()
                .filter_map(|(i, s)| (i != index).then_some(s))
                .collect();

            (Some(date), modified)
        }
        None => (None, args),
    }
}
