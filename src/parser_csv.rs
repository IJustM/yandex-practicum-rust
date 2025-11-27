use std::io::{ Read, Write };

use crate::{ Parser, Transaction, TxStatus, TxType, Col, error::{ ReadError, WriteError } };

#[derive(Debug)]
pub struct ParserCsv {
    pub transactions: Vec<Transaction>,
}

fn get_title_row() -> String {
    let row = format!(
        "{}",
        [
            Col::TxId,
            Col::TxType,
            Col::FromUserId,
            Col::ToUserId,
            Col::Amount,
            Col::Timestamp,
            Col::Status,
            Col::Description,
        ]
            .map(|c| c.to_string())
            .join(",")
    );
    row
}

impl Parser for ParserCsv {
    fn from_read<R: Read>(r: &mut R) -> Result<Self, ReadError> {
        let mut content = String::new();
        r.read_to_string(&mut content).map_err(|_| ReadError::Read)?;

        let mut lines = content.lines();

        match lines.next() {
            Some(first) if first == get_title_row() => {}
            _ => Err(ReadError::Title)?,
        }

        let transactions: Result<Vec<Transaction>, ReadError> = lines
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
            Ok(transactions) => Ok(Self { transactions }),
            Err(error) => Err(error),
        }
    }

    fn write_to<W: Write>(&mut self, writer: &mut W) -> Result<(), WriteError> {
        writeln!(writer, "{}", get_title_row()).map_err(|_| WriteError::Write)?;
        for t in &self.transactions {
            let line = format!(
                "{},{},{},{},{},{},{},\"{}\"",
                t.tx_id,
                t.tx_type,
                t.from_user_id,
                t.to_user_id,
                t.amount,
                t.timestamp,
                t.status,
                t.description
            );
            writeln!(writer, "{}", line).map_err(|_| WriteError::Write)?;
        }
        writer.flush().map_err(|_| WriteError::Write)?;
        Ok(())
    }
}
