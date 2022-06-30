use std::env;
use std::path::Path;
use std::fs::File;
use std::io::stdout;
use csv;

use csv_processing::{Transaction, Clients, MyError};

fn main() -> Result<(), MyError> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(MyError::WrongArguments())
    }

    let path = Path::new(&args[1]);

    process(&path)
}

fn process(input: &Path) -> Result<(), MyError> {
    let file = File::open(input)?;
    let mut rdr = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .flexible(true)
        .has_headers(true)
        .from_reader(file);

    let mut clients = Clients::new();

    for result in rdr.deserialize() {
        let transaction: Transaction = result?;

        clients.apply(transaction)?;
    }

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(stdout());
    for client in clients {
        wtr.serialize(client)?;
    }
    wtr.flush()?;

    Ok(())
}
