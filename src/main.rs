use std::env;
use timereport;

fn main() {
    let args: Vec<_> = env::args().collect();
    let output = timereport::main(args);
    println!("{output}");
}
