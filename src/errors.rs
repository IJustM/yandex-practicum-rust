use std::fmt;

use crate::parsers::{ bin::error::BinError, csv::error::CsvError, txt::error::TxtError };

/// Ошибка записи
#[derive(Debug)]
pub enum WriteError {
    /// Ошибка записи
    Write,
}

impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Write => write!(f, "Ошибка записи"),
        }
    }
}

impl std::error::Error for WriteError {}

/// Ошибка работы парсера
#[derive(Debug)]
pub enum ParserError {
    /// Неизвестное расширение файла
    UnknownExt,
    /// Ошибка открытия файла
    FileOpen,
    /// Ошибка создания файла
    FileCreate,
    /// Ошибка парсера
    Error(String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownExt => write!(f, "Неизвестное расширение файла"),
            Self::FileOpen => write!(f, "Ошибка открытия файла"),
            Self::FileCreate => write!(f, "Ошибка создания файла"),
            Self::Error(error) => write!(f, "Ошибка парсера. {error}"),
        }
    }
}

impl std::error::Error for ParserError {}

impl From<CsvError> for ParserError {
    fn from(value: CsvError) -> Self {
        ParserError::Error(value.to_string())
    }
}

impl From<TxtError> for ParserError {
    fn from(value: TxtError) -> Self {
        ParserError::Error(value.to_string())
    }
}

impl From<BinError> for ParserError {
    fn from(value: BinError) -> Self {
        ParserError::Error(value.to_string())
    }
}

impl From<WriteError> for ParserError {
    fn from(value: WriteError) -> Self {
        ParserError::Error(value.to_string())
    }
}
