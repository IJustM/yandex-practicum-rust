use std::{ fmt };

use crate::Field;

#[derive(Debug)]
pub enum TxtError {
    Read,
    AlreadyExists {
        field: Field,
    },
}

impl fmt::Display for TxtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read => write!(f, "Ошибка чтения"),
            Self::AlreadyExists { field } => write!(f, "Поле {} уже было прочитано", field),
        }
    }
}
