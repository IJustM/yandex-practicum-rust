use thiserror::Error;

use crate::Field;

/// Возможные ошибки при парсинге csv формата
#[derive(Debug, Error)]
pub enum CsvError {
    /// Ошибка чтения
    #[error("Ошибка чтения")]
    Read,
    /// Ошибка в заголовке
    #[error("Некорректный заголовок")]
    Header,
    /// Некорректное количество элементов в строке
    #[error("Некорректное количество элементов в строке {index}")]
    Length {
        /// Индекс строки
        index: usize,
    },
    /// Некорректное поле
    #[error("Ошибка парсинга поля {field} в строке {index}")]
    InvalidField {
        /// Индекс строки
        index: usize,
        /// Поле
        field: Field,
    },
}
