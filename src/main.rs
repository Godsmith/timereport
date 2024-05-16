use std::{env, path::Path};
use timereport;

fn main() {
    let path = Path::new("C:\\Users\\Filip\\Dropbox\\timereport.json");
    let args: Vec<_> = env::args().collect();
    let output = timereport::main(args, path);
    println!("{output}");
}
