use std::{ fmt };

use crate::Field;

#[derive(Debug)]
pub enum CsvError {
    Read,
    Header,
    Length {
        index: usize,
    },
    InvalidField {
        index: usize,
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
