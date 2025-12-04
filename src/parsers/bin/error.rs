use thiserror::Error;

use crate::Field;

/// Возможные ошибки при парсинге bin формата
#[derive(Debug, Error)]
pub enum BinError {
    /// Неизвестная ошибка
    #[error("Неизвестная ошибка")]
    Unknown,
    /// Ошибка чтения
    #[error("Ошибка чтения")]
    Read,
    /// Неожиданное завершение записи
    #[error("Неожиданное завершение записи {index}")]
    InvalidLength {
        /// Индекс строки
        index: usize,
    },
    /// Некорректный MAGIC
    #[error("Некорректный MAGIC в записи {index}")]
    InvalidMagic {
        /// Индекс строки
        index: usize,
    },
    /// Некорректный RECORD_SIZE
    #[error("Некорректный RECORD_SIZE в записи {index}")]
    InvalidRecordSize {
        /// Индекс строки
        index: usize,
    },
    /// Некорректное поле
    #[error("Ошибка парсинга поля {field} в записи {index}")]
    InvalidField {
        /// Индекс строки
        index: usize,
        /// Поле
        field: Field,
    },
    /// Некорректный DESC_LEN
    #[error("Некорректный DESC_LEN в записи {index}")]
    InvalidDescLen {
        /// Индекс строки
        index: usize,
    },
}
