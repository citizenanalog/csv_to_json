use serde::Deserialize;
use serde_json::to_string;
use csv::Reader;
use std::fs::File;
use std::io::prelude::*;
use std::env;

#[derive(Deserialize, Debug, PartialEq, Eq, serde::Serialize)]
struct Register {
    reg_type: String,
    addr: u32,
    descrip: String,
}

enum MapElement {
    regtype(String),
    regaddr(u16),
    regdescrip(String)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path: std::path::PathBuf = env::current_dir()?;
    println!("The current directory is {}", path.display());
    let mut file: File = File::open("map5.csv").unwrap();
    let mut csv_string: String = String::new();
    file.read_to_string(&mut csv_string).unwrap();
    let csv_bytes: &[u8] = csv_string.as_bytes();
    let mut rdr: Reader<&[u8]> = Reader::from_reader(csv_bytes);
    //#TODO - handle empty fields
    let records: Vec<Register> = rdr.deserialize().collect::<Result<_, _>>()?;
    let _json: String = to_string(&records)?;
    find_by_reg(32, &records);
    //print_all(&records);
    /*for i in 1..records.len() {
        println!("records: {:#?}", records[i]);
    }*/
    Ok(())
}

fn print_all(rec: &Vec<Register>) {
    for i in 1..rec.len() {
        println!("records: {:#?}", rec[i]);
    }
}

fn find_by_reg(n: u32, rec: &Vec<Register>) {
    for i in 1..rec.len() {
        if rec[i].addr == n {
            println!("found record: {:#?}", rec[i]);
        }
    }
}
