use std::{ fs };
use anyhow::Result;

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

fn main() -> Result<()> {
    let args = Args::parse();

    let Args { from, to } = args;

    let from_ext = from.parse::<ParserType>()?;
    let to_ext = to.parse::<ParserType>()?;

    let mut reader = fs::File::open(&from)?;

    let transaction = match from_ext {
        ParserType::Csv => CsvParser::from_read(&mut reader)?,
        ParserType::Txt => TxtParser::from_read(&mut reader)?,
        ParserType::Bin => BinParser::from_read(&mut reader)?,
    };

    let mut writer = fs::File::create(&to)?;

    match to_ext {
        ParserType::Csv => CsvParser::write_to(&mut writer, &transaction)?,
        ParserType::Txt => TxtParser::write_to(&mut writer, &transaction)?,
        ParserType::Bin => BinParser::write_to(&mut writer, &transaction)?,
    }

    println!("Конвертация успешно завершена!");
    Ok(())
}
