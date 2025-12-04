use std::fmt;

use crate::Field;

/// Возможные ошибки при парсинге bin формата
#[derive(Debug)]
pub enum BinError {
    /// Неизвестная ошибка
    Unknown,
    /// Ошибка чтения
    Read,
    /// Неожиданное завершение записи
    InvalidLength {
        /// Индекс строки
        index: usize,
    },
    /// Некорректный MAGIC
    InvalidMagic {
        /// Индекс строки
        index: usize,
    },
    /// Некорректный RECORD_SIZE
    InvalidRecordSize {
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
    /// Некорректный DESC_LEN
    InvalidDescLen {
        /// Индекс строки
        index: usize,
    },
}

impl fmt::Display for BinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Неизвестная ошибка"),
            Self::Read => write!(f, "Ошибка чтения"),
            Self::InvalidLength { index } => write!(f, "Неожиданное завершение записи {index}"),
            Self::InvalidMagic { index } => write!(f, "Некорректный MAGIC в записи {index}"),
            Self::InvalidRecordSize { index } =>
                write!(f, "Некорректный RECORD_SIZE в записи {index}"),
            Self::InvalidDescLen { index } => write!(f, "Некорректный DESC_LEN в записи {index}"),
            Self::InvalidField { field, index } =>
                write!(f, "Ошибка парсинга поля {field} в записи {index}"),
        }
    }
}

impl std::error::Error for BinError {}
