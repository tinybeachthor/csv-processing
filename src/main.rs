use std::env;
use std::path::Path;
use std::io;
use std::fs::File;

use csv;

use csv_processing::Transaction;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(io::Error::new(
                io::ErrorKind::Other,
                "Expecting 1 argument: path to the transactions file."));
    }

    let path = Path::new(&args[1]);

    process(&path)
}

fn process(input: &Path) -> io::Result<()> {
    let file = File::open(input)?;
    let mut rdr = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .flexible(true)
        .has_headers(true)
        .from_reader(file);
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(io::stdout());

    for result in rdr.deserialize() {
        let transaction: Transaction = result?;

        wtr.serialize(transaction)?;
    }

    wtr.flush()?;
    Ok(())
}
