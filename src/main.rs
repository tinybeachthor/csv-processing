use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Expecting 1 argument: path to the transactions file.");
        return;
    }

    let path = Path::new(&args[1]);
    println!("{:?}", path);
}
