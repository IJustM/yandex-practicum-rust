#![warn(missing_docs)]
//! Парсер

/// Парсеры
pub mod parsers;

/// Ошибки
pub mod errors;

use std::{ fs, io::{ Read, Write } };

use strum::{ Display, EnumString };

use crate::{
    errors::{ ParserError, WriteError },
    parsers::{ bin::bin::BinParser, csv::csv::CsvParser, txt::txt::TxtParser },
};

/// Виды парсеров
#[derive(Debug, Display, EnumString)]
pub enum ParserType {
    /// csv
    #[strum(serialize = "csv")]
    Csv,
    /// txt
    #[strum(serialize = "txt")]
    Txt,
    /// bin
    #[strum(serialize = "bin")]
    Bin,
}

impl ParserType {
    fn get_ext(value: &str) -> Result<Self, ParserError> {
        let ext = value.split(".").last().ok_or(ParserError::UnknownExt)?;
        let parser_type = ext.parse::<ParserType>().map_err(|_| ParserError::UnknownExt)?;
        Ok(parser_type)
    }
}

/// Поля транзакции
#[derive(Debug, Clone, PartialEq, Display)]
pub enum Field {
    /// Уникальный идентификатор транзакции
    #[strum(serialize = "TX_ID")]
    TxId,
    /// Тип транзакции
    #[strum(serialize = "TX_TYPE")]
    TxType,
    /// Идентификатор пользователя-отправителя
    #[strum(serialize = "FROM_USER_ID")]
    FromUserId,
    /// Идентификатор пользователя-получателя
    #[strum(serialize = "TO_USER_ID")]
    ToUserId,
    /// Сумма транзакции в наименьших единицах валюты
    #[strum(serialize = "AMOUNT")]
    Amount,
    /// Время совершения транзакции в формате Unix-времени
    #[strum(serialize = "TIMESTAMP")]
    Timestamp,
    /// Статус транзакции
    #[strum(serialize = "STATUS")]
    Status,
    /// Текстовое описание транзакции
    #[strum(serialize = "DESCRIPTION")]
    Description,
}

impl Field {
    fn get_all() -> [Field; 8] {
        [
            Self::TxId,
            Self::TxType,
            Self::FromUserId,
            Self::ToUserId,
            Self::Amount,
            Self::Timestamp,
            Self::Status,
            Self::Description,
        ]
    }
}

/// Тип транзакции
#[derive(Debug, Default, PartialEq, Display, EnumString)]
pub enum TxType {
    /// Поступление
    #[default]
    #[strum(serialize = "DEPOSIT")]
    Deposit,
    /// Перевод
    #[strum(serialize = "TRANSFER")]
    Transfer,
    /// Снятие
    #[strum(serialize = "WITHDRAWAL")]
    Withdrawal,
}

/// Статус транзакции
#[derive(Debug, Default, PartialEq, Display, EnumString)]
pub enum Status {
    /// Успешная
    #[default]
    #[strum(serialize = "SUCCESS")]
    Success,
    /// Не успешная
    #[strum(serialize = "FAILURE")]
    Failure,
    /// В процессе
    #[strum(serialize = "PENDING")]
    Pending,
}

/// Транзакция
#[derive(Debug, Default, PartialEq)]
pub struct Transaction {
    /// Уникальный идентификатор транзакции
    tx_id: u64,
    /// Тип транзакции
    tx_type: TxType,
    /// Идентификатор пользователя-отправителя
    from_user_id: u64,
    /// Идентификатор пользователя-получателя
    to_user_id: u64,
    /// Сумма транзакции в наименьших единицах валюты
    amount: u64,
    /// Время совершения транзакции в формате Unix-времени
    timestamp: i64,
    /// Статус транзакции
    status: Status,
    /// Текстовое описание транзакции
    description: String,
}

impl Transaction {
    fn get_value(&self, field: &Field) -> String {
        match field {
            Field::TxId => self.tx_id.to_string(),
            Field::TxType => self.tx_type.to_string(),
            Field::FromUserId => self.from_user_id.to_string(),
            Field::ToUserId => self.to_user_id.to_string(),
            Field::Amount => self.amount.to_string(),
            Field::Timestamp => self.timestamp.to_string(),
            Field::Status => self.status.to_string(),
            Field::Description => self.description.to_string(),
        }
    }
}

/// Парсер
pub trait Parser {
    /// Ошибка чтения
    type Error;

    /// Чтение транзаций из файла
    fn from_read<R: Read>(r: &mut R) -> Result<Vec<Transaction>, Self::Error>;

    /// Запись транзаций в файл
    fn write_to<W: Write>(
        writer: &mut W,
        transactions: &Vec<Transaction>
    ) -> Result<(), WriteError>;
}

/// Чтение транзаций из файла
pub fn from_read(from: &str) -> Result<Vec<Transaction>, ParserError> {
    let from_ext = ParserType::get_ext(from)?;

    let mut reader = fs::File::open(&from).map_err(|_| ParserError::FileOpen)?;

    let transactions = match from_ext {
        ParserType::Csv => CsvParser::from_read(&mut reader)?,
        ParserType::Txt => TxtParser::from_read(&mut reader)?,
        ParserType::Bin => BinParser::from_read(&mut reader)?,
    };

    Ok(transactions)
}

/// Запись транзаций в файл
pub fn write_to(transactions: Vec<Transaction>, to: &str) -> Result<(), ParserError> {
    let to_ext = ParserType::get_ext(to)?;

    let mut writer = fs::File::create(&to).map_err(|_| ParserError::FileCreate)?;

    match to_ext {
        ParserType::Csv => CsvParser::write_to(&mut writer, &transactions)?,
        ParserType::Txt => TxtParser::write_to(&mut writer, &transactions)?,
        ParserType::Bin => BinParser::write_to(&mut writer, &transactions)?,
    }

    Ok(())
}
