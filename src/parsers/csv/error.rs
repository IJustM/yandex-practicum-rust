use std::{ fmt };

use crate::Field;

#[derive(Debug)]
pub enum CsvError {
    Read,
    Title,
    Length {
        index: usize,
    },
    ParseField {
        index: usize,
        field: Field,
    },
}

impl fmt::Display for CsvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read => write!(f, "Ошибка чтения"),
            Self::Title => write!(f, "Некорректный заголовок"),
            Self::Length { index } =>
                write!(f, "Некорректное количество элементов в строке {}", index),
            Self::ParseField { index, field } =>
                write!(f, "Ошибка парсинга поля {} в строке {}", field, index),
        }
    }
}
