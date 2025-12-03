#![warn(missing_docs)]
//! Парсер

/// Парсеры
pub mod parsers;

mod errors;

use std::{ fmt, fs, io::{ Read, Write }, str::FromStr };

use crate::{
    errors::{ ParserTypeError, WriteError },
    parsers::{ bin::bin::BinParser, csv::csv::CsvParser, txt::txt::TxtParser },
};

/// Виды парсеров
pub enum ParserType {
    /// csv
    Csv,
    /// txt
    Txt,
    /// bin
    Bin,
}

impl FromStr for ParserType {
    type Err = ParserTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(".").last().ok_or(ParserTypeError::UnknownExt)? {
            "csv" => Ok(ParserType::Csv),
            "txt" => Ok(ParserType::Txt),
            "bin" => Ok(ParserType::Bin),
            _ => Err(ParserTypeError::UnknownExt),
        }
    }
}

impl fmt::Display for ParserType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Csv => "csv",
            Self::Txt => "txt",
            Self::Bin => "bin",
        };
        write!(f, "{s}")
    }
}

/// Поля транзакции
#[derive(Debug, Clone, PartialEq)]
pub enum Field {
    /// Уникальный идентификатор транзакции
    TxId,
    /// Тип транзакции
    TxType,
    /// Идентификатор пользователя-отправителя
    FromUserId,
    /// Идентификатор пользователя-получателя
    ToUserId,
    /// Сумма транзакции в наименьших единицах валюты
    Amount,
    /// Время совершения транзакции в формате Unix-времени
    Timestamp,
    /// Статус транзакции
    Status,
    /// Текстовое описание транзакции
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

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::TxId => "TX_ID",
            Self::TxType => "TX_TYPE",
            Self::FromUserId => "FROM_USER_ID",
            Self::ToUserId => "TO_USER_ID",
            Self::Amount => "AMOUNT",
            Self::Timestamp => "TIMESTAMP",
            Self::Status => "STATUS",
            Self::Description => "DESCRIPTION",
        };
        write!(f, "{s}")
    }
}

/// Тип транзакции
#[derive(Debug, Default, PartialEq)]
pub enum TxType {
    /// Поступление
    #[default]
    Deposit,
    /// Перевод
    Transfer,
    /// Снятие
    Withdrawal,
}

impl FromStr for TxType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DEPOSIT" => Ok(TxType::Deposit),
            "TRANSFER" => Ok(TxType::Transfer),
            "WITHDRAWAL" => Ok(TxType::Withdrawal),
            _ => Err(()),
        }
    }
}

impl fmt::Display for TxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Deposit => "DEPOSIT",
            Self::Transfer => "TRANSFER",
            Self::Withdrawal => "WITHDRAWAL",
        };
        write!(f, "{s}")
    }
}

/// Статус транзакции
#[derive(Debug, Default, PartialEq)]
pub enum Status {
    /// Успешная
    #[default]
    Success,
    /// Не успешная
    Failure,
    /// В процессе
    Pending,
}

impl FromStr for Status {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SUCCESS" => Ok(Status::Success),
            "FAILURE" => Ok(Status::Failure),
            "PENDING" => Ok(Status::Pending),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Success => "SUCCESS",
            Self::Failure => "FAILURE",
            Self::Pending => "PENDING",
        };
        write!(f, "{s}")
    }
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
pub fn from_read(from: &str) -> anyhow::Result<Vec<Transaction>> {
    let from_ext = from.parse::<ParserType>()?;

    let mut reader = fs::File::open(&from)?;

    let transactions = match from_ext {
        ParserType::Csv => CsvParser::from_read(&mut reader)?,
        ParserType::Txt => TxtParser::from_read(&mut reader)?,
        ParserType::Bin => BinParser::from_read(&mut reader)?,
    };

    Ok(transactions)
}

/// Запись транзаций в файл
pub fn write_to(transactions: Vec<Transaction>, to: &str) -> anyhow::Result<()> {
    let to_ext = to.parse::<ParserType>()?;

    let mut writer = fs::File::create(&to)?;

    match to_ext {
        ParserType::Csv => CsvParser::write_to(&mut writer, &transactions)?,
        ParserType::Txt => TxtParser::write_to(&mut writer, &transactions)?,
        ParserType::Bin => BinParser::write_to(&mut writer, &transactions)?,
    }

    Ok(())
}
