use std::fmt;

#[derive(Debug)]
pub enum ReadError {
    Length,
    ParseCol {
        name: String,
    },
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReadError::Length => write!(f, "Некорректное количество элементов в строке"),
            ReadError::ParseCol { name } => write!(f, "Ошибка парсинга поля {}", name),
        }
    }
}

#[derive(Debug)]
pub enum WriteError {}
