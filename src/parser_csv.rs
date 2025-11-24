use std::io::{ Read, Write };

use crate::{ Data, Parser, Transaction, TxStatus, TxType, Col, error::{ ReadError, WriteError } };

pub struct ParserCsv;

impl Parser for ParserCsv {
    fn from_read<R: Read>(r: &mut R) -> Result<Data, ReadError> {
        let mut content = String::new();
        let _ = r.read_to_string(&mut content);

        let transactions: Result<Vec<Transaction>, ReadError> = content
            .lines()
            .skip(1)
            .enumerate()
            .map(|(index, line)| {
                let row: Vec<&str> = line.split(",").collect();

                if row.len() != 8 {
                    return Err(ReadError::Length { index });
                }

                let parse_col_u64 = |i: usize, col: Col| {
                    row[i].parse::<u64>().map_err(|_| ReadError::ParseCol { index, col })
                };
                let parse_col_i64 = |i: usize, col: Col| {
                    row[i].parse::<i64>().map_err(|_| ReadError::ParseCol { index, col })
                };

                let tx_id = parse_col_u64(0, Col::TxId)?;
                let tx_type = row[1]
                    .parse::<TxType>()
                    .map_err(|_| ReadError::ParseCol { index, col: Col::TxType })?;
                let from_user_id: u64 = parse_col_u64(2, Col::FromUserId)?;
                let to_user_id: u64 = parse_col_u64(3, Col::ToUserId)?;
                let amount: u64 = parse_col_u64(4, Col::Amount)?;
                let timestamp: i64 = parse_col_i64(5, Col::Timestamp)?;
                let status = row[6]
                    .parse::<TxStatus>()
                    .map_err(|_| ReadError::ParseCol { index, col: Col::Status })?;
                let description = row[7].trim_matches('"').to_string();

                Ok(Transaction {
                    tx_id,
                    tx_type,
                    from_user_id,
                    to_user_id,
                    amount,
                    timestamp,
                    status,
                    description,
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
