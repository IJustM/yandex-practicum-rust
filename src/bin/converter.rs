use std::fs;

use clap::Parser;
use yandex_practicum_rust::{
    Parser as YP_Parser,
    ParserType,
    parsers::{ bin::bin::BinParser, csv::csv::CsvParser, txt::txt::TxtParser },
};

/// Программа для конвертации
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Файл, который будет конвертирован
    #[arg(short, long)]
    from: String,

    /// Файл, который будет создан
    #[arg(short, long)]
    to: String,
}

fn main() {
    let args = Args::parse();

    let Args { from, to } = args;

    let from_ext = from.parse::<ParserType>().unwrap_or_else(|e| panic!("{e}"));
    let to_ext = to.parse::<ParserType>().unwrap_or_else(|e| panic!("{e}"));

    let mut reader = fs::File::open(&from).unwrap_or_else(|_| panic!("Ошибка чтения файла {from}"));

    let transaction = match from_ext {
        ParserType::Csv => CsvParser::from_read(&mut reader).unwrap_or_else(|e| panic!("{e}")),
        ParserType::Txt => TxtParser::from_read(&mut reader).unwrap_or_else(|e| panic!("{e}")),
        ParserType::Bin => BinParser::from_read(&mut reader).unwrap_or_else(|e| panic!("{e}")),
    };

    let mut writer = fs::File::create(&to).unwrap_or_else(|_| panic!("Ошибка создания файла {to}"));

    match to_ext {
        ParserType::Csv =>
            CsvParser::write_to(&mut writer, &transaction).unwrap_or_else(|e| panic!("{e}")),
        ParserType::Txt =>
            TxtParser::write_to(&mut writer, &transaction).unwrap_or_else(|e| panic!("{e}")),
        ParserType::Bin =>
            BinParser::write_to(&mut writer, &transaction).unwrap_or_else(|e| panic!("{e}")),
    }

    println!("Конвертация успешно завершена!");
}
