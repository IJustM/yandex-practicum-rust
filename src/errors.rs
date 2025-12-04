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
    Csv(CsvError),
    /// Ошибка txt парсера
    #[error("Ошибка txt парсера: {0}")]
    Txt(TxtError),
    /// Ошибка bin парсера
    #[error("Ошибка bin парсера: {0}")]
    Bin(BinError),
    /// Ошибка записи
    #[error("Ошибка записи: {0}")]
    Write(WriteError),
}

impl From<CsvError> for ParserError {
    fn from(value: CsvError) -> Self {
        ParserError::Csv(value)
    }
}

impl From<TxtError> for ParserError {
    fn from(value: TxtError) -> Self {
        ParserError::Txt(value)
    }
}

impl From<BinError> for ParserError {
    fn from(value: BinError) -> Self {
        ParserError::Bin(value)
    }
}

impl From<WriteError> for ParserError {
    fn from(value: WriteError) -> Self {
        ParserError::Write(value)
    }
}
