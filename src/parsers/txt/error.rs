use std::{ fmt };

use crate::Field;

/// Возможные ошибки при парсинге txt формата
#[derive(Debug)]
pub enum TxtError {
    /// Неизвестная ошибка
    Unknown,
    /// Ошибка чтения
    Read,
    /// Некорректный формат в строке
    LineFormat {
        /// Индекс строки
        index: usize,
    },
    /// Неизвестное поле
    UnknownField {
        /// Индекс строки
        index: usize,
    },
    /// Повторное чтение поля
    FieldAlreadyExists {
        /// Индекс строки
        index: usize,
        /// Поле
        field: Field,
    },
    /// Пропущено поле
    MissingField {
        /// Индекс строки
        index: usize,
        /// Поле
        field: Field,
    },
    /// Некорректное поле
    InvalidField {
        /// Индекс строки
        index: usize,
        /// Поле
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
            Self::InvalidField { index, field } =>
                write!(f, "Ошибка парсинга поля {field} в строке {index}"),
        }
    }
}

impl std::error::Error for TxtError {}
