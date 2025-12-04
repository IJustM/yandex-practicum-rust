use std::{ fmt };

use crate::Field;

/// Возможные ошибки при парсинге csv формата
#[derive(Debug)]
pub enum CsvError {
    /// Ошибка чтения
    Read,
    /// Ошибка в заголовке
    Header,
    /// Некорректное количество элементов в строке
    Length {
        /// Индекс строки
        index: usize,
    },
    /// Некорректное поле
    InvalidField {
        /// Индекс строки
        index: usize,
        /// Поле
        field: Field,
    },
}

impl fmt::Display for CsvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read => write!(f, "Ошибка чтения"),
            Self::Header => write!(f, "Некорректный заголовок"),
            Self::Length { index } =>
                write!(f, "Некорректное количество элементов в строке {index}"),
            Self::InvalidField { index, field } =>
                write!(f, "Ошибка парсинга поля {field} в строке {index}"),
        }
    }
}

impl std::error::Error for CsvError {}
