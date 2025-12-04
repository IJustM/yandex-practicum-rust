use thiserror::Error;

use crate::Field;

/// Возможные ошибки при парсинге txt формата
#[derive(Debug, Error)]
pub enum TxtError {
    /// Неизвестная ошибка
    #[error("Неизвестная ошибка")]
    Unknown,
    /// Ошибка чтения
    #[error("Ошибка чтения")]
    Read,
    /// Некорректный формат в строке
    #[error("Некорректный формат в строке {index}")]
    LineFormat {
        /// Индекс строки
        index: usize,
    },
    /// Неизвестное поле
    #[error("Неизвестное поле в строке {index}")]
    UnknownField {
        /// Индекс строки
        index: usize,
    },
    /// Повторное чтение поля
    #[error("Повторное чтение поле {field} в строке {index}")]
    FieldAlreadyExists {
        /// Индекс строки
        index: usize,
        /// Поле
        field: Field,
    },
    /// Пропущено поле
    #[error("Отсутствует поле {field} в записи на строке {index}")]
    MissingField {
        /// Индекс строки
        index: usize,
        /// Поле
        field: Field,
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
