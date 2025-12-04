use std::fs;

use clap::Parser;
use yandex_practicum_rust::{from_read, write_to};

/// Программа для конвертации
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Файл, который будет конвертирован
    #[arg(long)]
    from: String,

    /// Файл, который будет создан
    #[arg(long)]
    to: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let Args { from, to } = args;

    let mut reader = fs::File::open(&from).expect("Ошибка чтения файла");

    let transactions = from_read(&mut reader, &from)?;

    let mut writer = fs::File::create(&to).expect("Ошибка создания файла");

    write_to(&mut writer, &transactions, &to)?;

    println!("Конвертация успешно завершена!");
    Ok(())
}
