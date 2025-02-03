use std::{env, path::Path};
// Rust note: the binary is a separate crate from the library, so we must import
// using the full name of the library crate here, not just "crate"
use timereport;

fn main() {
    let path = Path::new("C:\\Users\\Filip\\Dropbox\\timereport.json");
    // Skip the first argument since it is just the file
    let args: Vec<_> = env::args().skip(1).collect();
    let output = timereport::main(args, path);
    println!("{output}");
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn call_main_function_for_code_coverage() {
        main();
    }
}
