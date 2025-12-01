use std::fmt;

#[derive(Debug)]
pub enum WriteError {
    Write,
}

impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Write => write!(f, "Ошибка записи"),
        }
    }
}

pub enum ParserTypeError {
    UnknownExt,
}

impl fmt::Display for ParserTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownExt => write!(f, "Неизвестное расширение файла"),
        }
    }
}
