use std::io::{ Read, Write };

use crate::{
    Field,
    Parser,
    Transaction,
    TxStatus,
    TxType,
    errors::WriteError,
    parsers::txt::error::TxtError,
};

pub struct TxtParser;

impl Parser for TxtParser {
    type Error = TxtError;

    fn from_read<R: Read>(r: &mut R) -> Result<Vec<Transaction>, TxtError> {
        let mut content = String::new();
        r.read_to_string(&mut content).map_err(|_| TxtError::Read)?;

        let mut transactions: Vec<Transaction> = vec![];

        let lines = content.lines().filter(|&line| !line.starts_with("#"));

        let mut exists = [Field::TxId];

        let mut tx_type_exists = false;
        for line in lines {
            if line.starts_with(&Field::TxId.to_string()) {
                if tx_type_exists {
                    return Err(TxtError::AlreadyExists { field: Field::TxId });
                }
                tx_type_exists = true;
            }
        }

        Ok(transactions)
    }

    fn write_to<W: Write>(
        transactions: &Vec<Transaction>,
        writer: &mut W
    ) -> Result<(), WriteError> {
        Ok(())
    }
}
