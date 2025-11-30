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

        let mut transactions: Vec<Transaction> = vec![];

        for (index, line) in content.lines().enumerate() {
            if index == 0 {
                if line == get_header_row() {
                    continue;
                } else {
                    return Err(CsvError::Header);
                }
            }

            if line.is_empty() {
                continue;
            }

            let values: Vec<&str> = line.split(",").collect();

            if values.len() != 8 {
                return Err(CsvError::Length { index });
            }

            let parse_col_u64 = |i: usize, field: Field| {
                values[i].parse::<u64>().map_err(|_| CsvError::ParseField { index, field })
            };
            let parse_col_i64 = |i: usize, field: Field| {
                values[i].parse::<i64>().map_err(|_| CsvError::ParseField { index, field })
            };

            transactions.push(Transaction {
                tx_id: parse_col_u64(0, Field::TxId)?,
                tx_type: values[1]
                    .parse::<TxType>()
                    .map_err(|_| CsvError::ParseField { index, field: Field::TxType })?,
                from_user_id: parse_col_u64(2, Field::FromUserId)?,
                to_user_id: parse_col_u64(3, Field::ToUserId)?,
                amount: parse_col_u64(4, Field::Amount)?,
                timestamp: parse_col_i64(5, Field::Timestamp)?,
                status: values[6]
                    .parse::<TxStatus>()
                    .map_err(|_| CsvError::ParseField { index, field: Field::Status })?,
                description: (if values[7].starts_with('"') && values[7].ends_with('"') {
                    Ok(values[7].trim_matches('"').to_string())
                } else {
                    Err(CsvError::ParseField { index, field: Field::Description })
                })?,
            });
        }

        Ok(transactions)
    }

    fn write_to<W: Write>(
        transactions: &Vec<Transaction>,
        writer: &mut W
    ) -> Result<(), WriteError> {
        writeln!(writer, "{}", get_header_row()).map_err(|_| WriteError::Write)?;
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
            writeln!(writer, "{line}").map_err(|_| WriteError::Write)?;
        }
        writer.flush().map_err(|_| WriteError::Write)?;
        Ok(())
    }
}

fn get_header_row() -> String {
    let row = Field::get_all()
        .map(|c| c.to_string())
        .join(",");
    row
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_success_parse() {
        let mut cursor = get_cursor("0,DEPOSIT,0,1,100,1633036860000,SUCCESS,\"Test 1\"");
        let result = CsvParser::from_read(&mut cursor).unwrap();
        assert_eq!(result, [
            Transaction {
                tx_id: 0,
                tx_type: TxType::Deposit,
                from_user_id: 0,
                to_user_id: 1,
                amount: 100,
                timestamp: 1633036860000,
                status: TxStatus::Success,
                description: "Test 1".to_string(),
            },
        ]);
    }

    #[test]
    fn test_get_header_row() {
        assert_eq!(
            get_header_row(),
            "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION"
        );
    }

    #[test]
    fn test_error_header() {
        let data = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,DESCRIPTION,STATUS";
        let mut cursor = Cursor::new(data);
        let result = CsvParser::from_read(&mut cursor).unwrap_err();
        assert_eq!(result.to_string(), "Некорректный заголовок");
    }

    fn get_cursor(data: &str) -> Cursor<String> {
        let header = get_header_row();
        let cursor = Cursor::new(format!("{header}\n{data}"));
        cursor
    }

    #[test]
    fn test_error_length() {
        let mut cursor = get_cursor("0");
        let result = CsvParser::from_read(&mut cursor).unwrap_err();
        assert_eq!(result.to_string(), "Некорректное количество элементов в строке 1");
    }

    #[test]
    fn test_error_parse_field() {
        let mut cursor = get_cursor("!,DEPOSIT,0,1,100,1633036860000,FAILURE,\"Test\"");
        let result = CsvParser::from_read(&mut cursor).unwrap_err();
        assert_eq!(result.to_string(), "Ошибка парсинга поля TX_ID в строке 1");

        let mut cursor = get_cursor("0,!,0,1,100,1633036860000,FAILURE,\"Test\"");
        let result = CsvParser::from_read(&mut cursor).unwrap_err();
        assert_eq!(result.to_string(), "Ошибка парсинга поля TX_TYPE в строке 1");

        let mut cursor = get_cursor("0,DEPOSIT,!,1,100,1633036860000,FAILURE,\"Test\"");
        let result = CsvParser::from_read(&mut cursor).unwrap_err();
        assert_eq!(result.to_string(), "Ошибка парсинга поля FROM_USER_ID в строке 1");

        let mut cursor = get_cursor("0,DEPOSIT,0,!,100,1633036860000,FAILURE,\"Test\"");
        let result = CsvParser::from_read(&mut cursor).unwrap_err();
        assert_eq!(result.to_string(), "Ошибка парсинга поля TO_USER_ID в строке 1");

        let mut cursor = get_cursor("0,DEPOSIT,0,1,!,1633036860000,FAILURE,\"Test\"");
        let result = CsvParser::from_read(&mut cursor).unwrap_err();
        assert_eq!(result.to_string(), "Ошибка парсинга поля AMOUNT в строке 1");

        let mut cursor = get_cursor("0,DEPOSIT,0,1,100,!,FAILURE,\"Test\"");
        let result = CsvParser::from_read(&mut cursor).unwrap_err();
        assert_eq!(result.to_string(), "Ошибка парсинга поля TIMESTAMP в строке 1");

        let mut cursor = get_cursor("0,DEPOSIT,0,1,100,1633036860000,!,\"Test\"");
        let result = CsvParser::from_read(&mut cursor).unwrap_err();
        assert_eq!(result.to_string(), "Ошибка парсинга поля STATUS в строке 1");

        let mut cursor = get_cursor("0,DEPOSIT,0,1,100,1633036860000,FAILURE,\'Test\'");
        let result = CsvParser::from_read(&mut cursor).unwrap_err();
        assert_eq!(result.to_string(), "Ошибка парсинга поля DESCRIPTION в строке 1");
    }
}
