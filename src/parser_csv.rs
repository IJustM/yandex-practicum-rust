use std::io::{ Read, Write };

use crate::{ Data, Parser, Transaction, TxStatus, TxType, error::{ ReadError, WriteError } };

pub struct ParserCsv;

impl Parser for ParserCsv {
    type Output = Data;

    fn from_read<R: Read>(r: &mut R) -> Result<Data, ReadError> {
        let mut content = String::new();
        let _ = r.read_to_string(&mut content);

        let transactions: Result<Vec<Transaction>, ReadError> = content
            .lines()
            .skip(1)
            .map(|t| {
                let row: Vec<&str> = t.split(",").collect();

                if row.len() != 8 {
                    return Err(ReadError::Length);
                }

                let tx_id: u64 = row[0]
                    .parse()
                    .map_err(|_| ReadError::ParseCol { name: "tx_id".to_string() })?;

                Ok(Transaction {
                    tx_id,
                    amount: 0,
                    description: "".to_string(),
                    from_user_id: 0,
                    status: TxStatus::Failure,
                    timestamp: 0,
                    to_user_id: 0,
                    tx_type: TxType::Deposit,
                })
            })
            .collect();

        match transactions {
            Ok(transactions) => Ok(Data { transactions }),
            Err(error) => Err(error),
        }
    }

    fn write_to<W: Write>(&mut self, writer: &mut W) -> Result<(), WriteError> {
        todo!()
    }
}
