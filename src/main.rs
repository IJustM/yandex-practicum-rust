use std::{ fs, io, path::Path };

use yandex_practicum_rust::{ Parser, parser_csv::ParserCsv };

fn main() {
    let mut reader = fs::File::open("files/records_example.csv").unwrap();
    let mut parser = ParserCsv::from_read(&mut reader).unwrap();

    let output = "files/records_example2.csv";
    if Path::new(output).exists() {
        fs::remove_file(output).unwrap();
    }
    let file = fs::File::create(output).unwrap();
    let mut writer = io::BufWriter::new(file);
    let _ = parser.write_to(&mut writer);

    // let mut reader = fs::File::open("files/records_example.txt").unwrap();
    // let mut parser = ParserCsv::from_read(&mut reader).unwrap();

    // let output = "files/records_example2.txt";
    // if Path::new(output).exists() {
    //     fs::remove_file(output).unwrap();
    // }
    // let file = fs::File::create(output).unwrap();
    // let mut writer = io::BufWriter::new(file);
    // let _ = parser.write_to(&mut writer);
}
