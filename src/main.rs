use serde::Deserialize;
use serde_json::to_string;
use csv::Reader;
use std::fs::File;
use std::io::prelude::*;
use std::env;

#[derive(Deserialize, Debug, PartialEq, Eq, serde::Serialize)]
struct Register {
    reg_type: String,
    addr: i32,
    descrip: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());
    let mut file = File::open("map5.csv").unwrap();
    let mut csv_string = String::new();
    file.read_to_string(&mut csv_string).unwrap();
    let csv_bytes = csv_string.as_bytes();
    let mut rdr = Reader::from_reader(csv_bytes);
    //#TODO - handle empty fields
    let records: Vec<Register> = rdr.deserialize().collect::<Result<_, _>>()?;
    let _json = to_string(&records)?;
    //print the first 10
    for i in 1..1000 {
        println!("records: {:#?}", records[i]);
    }
    //println!("{:#?}", json);
    Ok(())
}

