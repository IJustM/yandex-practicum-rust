use std::{ fmt };

use crate::Field;

#[derive(Debug)]
pub enum TxtError {
    Unknown,
    Read,
    LineFormat {
        index: usize,
    },
    UnknownField {
        index: usize,
    },
    FieldAlreadyExists {
        index: usize,
        field: Field,
    },
    MissingField {
        index: usize,
        field: Field,
    },
    ParseField {
        index: usize,
        field: Field,
    },
}

impl fmt::Display for TxtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Неизвестная ошибка"),
            Self::Read => write!(f, "Ошибка чтения"),
            Self::LineFormat { index } => write!(f, "Некорректный формат в строке {index}"),
            Self::UnknownField { index } => write!(f, "Неизвестное поле в строке {index}"),
            Self::FieldAlreadyExists { index, field } =>
                write!(f, "Повторное чтение поле {field} в строке {index}"),
            Self::MissingField { index, field } =>
                write!(f, "Отсутствует поле {field} в записи на строке {index}"),
            Self::ParseField { index, field } =>
                write!(f, "Ошибка парсинга поля {field} в строке {index}"),
        }
    }
}
