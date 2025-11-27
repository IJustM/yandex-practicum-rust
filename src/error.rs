use std::{ fmt };

use crate::Col;

#[derive(Debug)]
pub enum ReadError {
    Read,
    Title,
    Length {
        index: usize,
    },
    ParseCol {
        index: usize,
        col: Col,
    },
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReadError::Read => write!(f, "Ошибка чтения"),
            ReadError::Title => write!(f, "Некорректный заголовок"),
            ReadError::Length { index } =>
                write!(f, "Некорректное количество элементов в строке {}", index),
            ReadError::ParseCol { index, col } =>
                write!(f, "Ошибка парсинга поля {} в строке {}", col, index),
        }
    }
}

#[derive(Debug)]
pub enum WriteError {
    Write,
}

impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WriteError::Write => write!(f, "Ошибка записи"),
        }
    }
}
