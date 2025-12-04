use thiserror::Error;

use crate::parsers::{bin::error::BinError, csv::error::CsvError, txt::error::TxtError};

/// Ошибка записи
#[derive(Debug, Error)]
pub enum WriteError {
    /// Ошибка записи
    #[error("Ошибка записи")]
    Write,
}

/// Ошибка работы парсера
#[derive(Debug, Error)]
pub enum ParserError {
    /// Неизвестное расширение файла
    #[error("Неизвестное расширение файла")]
    UnknownExt,
    /// Ошибка csv парсера
    #[error("Ошибка csv парсера: {0}")]
    Csv(#[from] CsvError),
    /// Ошибка txt парсера
    #[error("Ошибка txt парсера: {0}")]
    Txt(#[from] TxtError),
    /// Ошибка bin парсера
    #[error("Ошибка bin парсера: {0}")]
    Bin(#[from] BinError),
    /// Ошибка записи
    #[error("Ошибка записи: {0}")]
    Write(#[from] WriteError),
}
