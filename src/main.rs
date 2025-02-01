use std::{env, path::Path};
// Rust note: the binary is a separate crate from the library, so we must import
// using the full name of the library crate here, not just "crate"
use timereport;

fn main() {
    let path = Path::new("C:\\Users\\Filip\\Dropbox\\timereport.json");
    let args: Vec<_> = env::args().collect();
    let output = timereport::main(args, path);
    println!("{output}");
}
