use std::error::Error;

fn main() {
    let args: Vec<_> = std::env::args().collect();

   if let Err(e) = read_from_file(&*args[1]) {
     eprintln!("{}", e);
   }
}

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut csv_reader = csv::Reader::from_path(path)?;

    for result in csv_reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
