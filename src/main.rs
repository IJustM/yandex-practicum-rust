use std::fs;

use yandex_practicum_rust::{ Parser, parser_csv::ParserCsv };

fn main() {
    let mut reader = fs::File::open("files/records_example.csv").unwrap();
    let data = ParserCsv::from_read(&mut reader).unwrap();
    for t in data.transactions {
        println!("t = {:?}", t);
    }
}
