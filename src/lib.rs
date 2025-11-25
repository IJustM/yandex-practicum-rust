pub mod error;
pub mod parser_csv;

use std::{ fmt, io::{ Read, Write }, str::FromStr };

use crate::error::{ ReadError, WriteError };

#[derive(Debug)]
pub enum Col {
    TxId,
    TxType,
    FromUserId,
    ToUserId,
    Amount,
    Timestamp,
    Description,
    Status,
}

impl fmt::Display for Col {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Col::TxId => "TX_ID",
            Col::TxType => "TX_TYPE",
            Col::FromUserId => "FROM_USER_ID",
            Col::ToUserId => "TO_USER_ID",
            Col::Amount => "AMOUNT",
            Col::Timestamp => "TIMESTAMP",
            Col::Description => "DESCRIPTION",
            Col::Status => "STATUS",
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
            TxType::Deposit => "DEPOSIT",
            TxType::Transfer => "TRANSFER",
            TxType::Withdrawal => "WITHDRAWAL",
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
            TxStatus::Success => "SUCCESS",
            TxStatus::Failure => "FAILURE",
            TxStatus::Pending => "PENDING",
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
    // чтение из файла
    fn from_read<R: Read>(r: &mut R) -> Result<Self, ReadError> where Self: Sized;

    // запись в файл
    fn write_to<W: Write>(&mut self, writer: &mut W) -> Result<(), WriteError>;
}

