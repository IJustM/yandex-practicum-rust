pub mod parsers;
mod errors;

use std::{ fmt, io::{ Read, Write }, str::FromStr };

use crate::errors::WriteError;

#[derive(Debug)]
pub enum Field {
    TxId,
    TxType,
    FromUserId,
    ToUserId,
    Amount,
    Timestamp,
    Description,
    Status,
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
            Self::Description,
            Self::Status,
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
            Self::Description => "DESCRIPTION",
            Self::Status => "STATUS",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum TxType {
    Deposit,
    Transfer,
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
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum TxStatus {
    Success,
    Failure,
    Pending,
}

impl FromStr for TxStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SUCCESS" => Ok(TxStatus::Success),
            "FAILURE" => Ok(TxStatus::Failure),
            "PENDING" => Ok(TxStatus::Pending),
            _ => Err(()),
        }
    }
}

impl fmt::Display for TxStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Success => "SUCCESS",
            Self::Failure => "FAILURE",
            Self::Pending => "PENDING",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub struct Transaction {
    tx_id: u64,
    tx_type: TxType,
    from_user_id: u64,
    to_user_id: u64,
    amount: u64,
    timestamp: i64,
    status: TxStatus,
    description: String,
}

pub trait Parser {
    type Error;

    // чтение из файла
    fn from_read<R: Read>(r: &mut R) -> Result<Vec<Transaction>, Self::Error>;

    // запись в файл
    fn write_to<W: Write>(
        transactions: &Vec<Transaction>,
        writer: &mut W
    ) -> Result<(), WriteError>;
}
