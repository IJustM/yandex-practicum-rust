use std::{ fs, io, path::Path };

use yandex_practicum_rust::{ Parser, parsers::{ csv::csv::CsvParser, txt::txt::TxtParser } };

fn main() {
    // CSV
    let mut reader = fs::File::open("files/records_example.csv").unwrap();
    let transactions = CsvParser::from_read(&mut reader).unwrap();

    let output = "files/records_example2.csv";
    if Path::new(output).exists() {
        fs::remove_file(output).unwrap();
    }
    let file = fs::File::create(output).unwrap();
    let mut writer = io::BufWriter::new(file);
    let _ = CsvParser::write_to(&transactions, &mut writer);

    // TXT
    let mut reader = fs::File::open("files/records_example.txt").unwrap();
    let transactions = TxtParser::from_read(&mut reader).unwrap();

    let output = "files/records_example2.txt";
    if Path::new(output).exists() {
        fs::remove_file(output).unwrap();
    }
    let file = fs::File::create(output).unwrap();
    let mut writer = io::BufWriter::new(file);
    let _ = TxtParser::write_to(&transactions, &mut writer);
}
