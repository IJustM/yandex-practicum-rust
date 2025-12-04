use std::fs;

use clap::Parser;
use yandex_practicum_rust::from_read;

/// Программа для конвертации
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Файл, который будет конвертирован
    #[arg(long)]
    file1: String,

    /// Файл, который будет создан
    #[arg(long)]
    file2: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let Args { file1, file2 } = args;

    let mut reader1 = fs::File::open(&file1).expect("Ошибка чтения файла file1");
    let mut reader2 = fs::File::open(&file2).expect("Ошибка чтения файла file2");

    let transactions1 = from_read(&mut reader1, &file1)?;
    let transactions2 = from_read(&mut reader2, &file2)?;

    println!(
        "{}",
        if transactions1 == transactions2 {
            "Данные совпадают"
        } else {
            "Данные не совпадают"
        }
    );

    Ok(())
}
