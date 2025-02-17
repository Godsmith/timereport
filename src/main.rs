use std::{env, path::PathBuf};
// Rust note: the binary is a separate crate from the library, so we must import
// using the full name of the library crate here, not just "crate"
use timereport;

fn main() {
    let path = match get_timereport_json_path() {
        Ok(path) => path,
        Err(message) => {
            println!("Error: {}", message);
            return;
        }
    };
    // Skip the first argument since it is just the file
    let args: Vec<_> = env::args().skip(1).collect();
    let output = timereport::main(args, &path);
    println!("{output}")
}

fn get_timereport_json_path() -> Result<PathBuf, String> {
    let path_string = if let Ok(timereport_path) = env::var("TIMEREPORT_PATH") {
        timereport_path
    } else {
        let user = if let Ok(user) = env::var("USER") {
            user
        } else if let Ok(username) = env::var("USERNAME") {
            username
        } else {
            return Err("TIMEREPORT_PATH, USER or USERNAME must be defined".to_string());
        };
        format!("C:\\Users\\{}\\Dropbox\\timereport.json", user)
    };
    let path = PathBuf::from(&path_string);
    let parent = match path.parent() {
        Some(parent) => parent,
        None => return Err(format!("Error when trying to find the parent directory of '{}'. Is TIMEREPORT_PATH set correctly?", path.to_string_lossy())),
    };
    if parent.exists() {
        Ok(path)
    } else {
        Err(format!("'{}' is not a directory. timereport.json cannot be created. Try setting the TIMEREPORT_PATH environment variable to a valid path.", parent.to_string_lossy()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn call_main_function_for_code_coverage() {
        main();
    }

    fn current_test_file_path() -> PathBuf {
        // Get the path of the current source file (relative to the project root)
        let file_path = file!();

        // Get the project's root directory (from the `CARGO_MANIFEST_DIR` env variable)
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR environment variable not set");

        // Combine them to get the absolute path of the current test file
        PathBuf::from(&manifest_dir)
            .join(file_path)
            .canonicalize()
            .expect("Failed to resolve path")
    }

    #[rstest]
    fn timereport_path_is_legal() {
        let test_file_path = current_test_file_path();
        let dir = test_file_path.parent().expect("should be legal");
        let timereport_json_path = dir.join("timereport.json");
        temp_env::with_var(
            "TIMEREPORT_PATH",
            Some(timereport_json_path.clone()),
            || assert!(get_timereport_json_path() == Ok(timereport_json_path)),
        );
    }

    #[rstest]
    fn timereport_path_is_illegal() {
        temp_env::with_vars([("TIMEREPORT_PATH", Some(""))], || {
            assert!(get_timereport_json_path()
                .expect_err("")
                .contains("Error when trying to find the parent directory"));
        });
    }

    #[rstest]
    fn user_is_set() {
        temp_env::with_vars([("USERNAME", None), ("USER", Some("MYUSER"))], || {
            assert!(get_timereport_json_path()
                .expect_err("")
                .contains("'C:\\Users\\MYUSER\\Dropbox' is not a directory"));
        });
    }

    #[rstest]
    fn username_is_set() {
        temp_env::with_vars([("USERNAME", Some("MYUSER")), ("USER", None)], || {
            assert!(get_timereport_json_path()
                .expect_err("")
                .contains("'C:\\Users\\MYUSER\\Dropbox' is not a directory"));
        });
    }

    #[rstest]
    fn nothing_is_set() {
        temp_env::with_vars(
            [
                ("TIMEREPORT_PATH", None::<String>),
                ("USERNAME", None),
                ("USER", None),
            ],
            || {
                assert!(get_timereport_json_path()
                    .expect_err("")
                    .contains("must be defined"));
            },
        );
    }
}
