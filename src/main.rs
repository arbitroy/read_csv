use std::error::Error;
use std::path::Path;
use std::fs::File;
use csv::ReaderBuilder;

fn main() {
    //create a vector called args to collect users input in the CLI
    let args: Vec<_> = std::env::args().collect();
     //if args less than 2, there's an issue because  you need to send the name of the 
    // csv file and it'll show you how to use
    if args.len() <2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }
    //Get file name
    let fname = std::path::Path::new(&*args[1]);
  if let Err(e) = read_from_file(fname){
    eprintln!("{}", e);
  }
}
fn read_from_file(path: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .from_reader(file);

    for result in reader.records(){
        let record = result?;
        //Process the record as needed
        println!("{:?}", record);
    }
    Ok(())
}
