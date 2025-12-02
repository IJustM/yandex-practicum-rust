use std::fmt;

use crate::Field;

#[derive(Debug)]
pub enum BinError {
    Unknown,
    Read,
    InvalidMagic {
        index: usize,
    },
    InvalidRecordSize {
        index: usize,
    },
    InvalidField {
        index: usize,
        field: Field,
    },
    InvalidDescLen {
        index: usize,
    },
}

impl fmt::Display for BinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Неизвестная ошибка"),
            Self::Read => write!(f, "Ошибка чтения"),
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
