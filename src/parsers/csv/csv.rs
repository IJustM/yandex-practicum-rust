use std::io::{ Read, Write };

use crate::{
    Field,
    Parser,
    Transaction,
    TxStatus,
    TxType,
    errors::WriteError,
    parsers::csv::error::CsvError,
};

pub struct CsvParser;

impl Parser for CsvParser {
    type Error = CsvError;

    fn from_read<R: Read>(r: &mut R) -> Result<Vec<Transaction>, CsvError> {
        let mut content = String::new();
        r.read_to_string(&mut content).map_err(|_| CsvError::Read)?;

        let mut lines = content.lines();

        match lines.next() {
            Some(first) if first == get_title_row() => {}
            _ => Err(CsvError::Title)?,
        }

        let transactions: Result<Vec<Transaction>, CsvError> = lines
            .enumerate()
            .map(|(index, line)| {
                let row: Vec<&str> = line.split(",").collect();

                if row.len() != 8 {
                    return Err(CsvError::Length { index });
                }

                let parse_col_u64 = |i: usize, field: Field| {
                    row[i].parse::<u64>().map_err(|_| CsvError::ParseCol { index, field })
                };
                let parse_col_i64 = |i: usize, field: Field| {
                    row[i].parse::<i64>().map_err(|_| CsvError::ParseCol { index, field })
                };

                let tx_id = parse_col_u64(0, Field::TxId)?;
                let tx_type = row[1]
                    .parse::<TxType>()
                    .map_err(|_| CsvError::ParseCol { index, field: Field::TxType })?;
                let from_user_id: u64 = parse_col_u64(2, Field::FromUserId)?;
                let to_user_id: u64 = parse_col_u64(3, Field::ToUserId)?;
                let amount: u64 = parse_col_u64(4, Field::Amount)?;
                let timestamp: i64 = parse_col_i64(5, Field::Timestamp)?;
                let status = row[6]
                    .parse::<TxStatus>()
                    .map_err(|_| CsvError::ParseCol { index, field: Field::Status })?;
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

        transactions
    }

    fn write_to<W: Write>(
        transactions: &Vec<Transaction>,
        writer: &mut W
    ) -> Result<(), WriteError> {
        writeln!(writer, "{}", get_title_row()).map_err(|_| WriteError::Write)?;
        for t in transactions {
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

fn get_title_row() -> String {
    let row = format!(
        "{}",
        Field::get_all()
            .map(|c| c.to_string())
            .join(",")
    );
    row
}
