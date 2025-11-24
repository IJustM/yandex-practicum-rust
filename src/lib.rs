pub mod error;
pub mod parser_csv;

use std::{ fmt, io::{ Read, Write } };

use crate::error::{ ReadError, WriteError };

#[derive(Debug)]
pub enum TxType {
    Deposit,
    Transfer,
    Withdrawal,
}

#[derive(Debug)]
pub enum TxStatus {
    Success,
    Failure,
    Pending,
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

#[derive(Debug)]
pub struct Data {
    pub transactions: Vec<Transaction>,
}

pub trait Parser {
    type Output;

    // чтение из файла
    fn from_read<R: Read>(r: &mut R) -> Result<Self::Output, ReadError>;

    // запись в файл
    fn write_to<W: Write>(&mut self, writer: &mut W) -> Result<(), WriteError>;
}
