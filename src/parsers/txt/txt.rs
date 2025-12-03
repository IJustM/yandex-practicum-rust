use std::{ io::{ Read, Write } };

use crate::{
    Field,
    Parser,
    Status,
    Transaction,
    TxType,
    errors::WriteError,
    parsers::{ txt::error::TxtError, utils::description_trim },
};

/// Парсер для txt формата
pub struct TxtParser;

impl Parser for TxtParser {
    type Error = TxtError;

    fn from_read<R: Read>(r: &mut R) -> Result<Vec<Transaction>, TxtError> {
        let mut content = String::new();
        r.read_to_string(&mut content).map_err(|_| TxtError::Read)?;

        let mut transactions: Vec<Transaction> = Vec::new();
        let mut transaction = Transaction::default();

        let mut parsed_fields = Field::get_all().map(|f| (f, false));

        let mut lines: Vec<&str> = content.lines().collect();
        lines.push(""); // добавляем пустую строку для обработки последней записи
        let last_index = lines.len() - 1;

        for (index, line) in lines.iter().enumerate() {
            if index == last_index || line.is_empty() {
                let missed_field = parsed_fields.iter().find(|f| !f.1);
                if let Some((field, _)) = missed_field {
                    if line.is_empty() && parsed_fields.iter().all(|f| !f.1) {
                        continue;
                    }
                    return Err(TxtError::MissingField { index, field: field.clone() });
                }
                parsed_fields.iter_mut().for_each(|f| {
                    f.1 = false;
                });
                transactions.push(transaction);
                transaction = Transaction::default();
                continue;
            }

            if line.starts_with("#") {
                continue;
            }

            let key_and_value: Vec<&str> = line.split(": ").collect();

            let [key, value] = key_and_value[..] else {
                return Err(TxtError::LineFormat { index });
            };

            let field = Field::get_all()
                .into_iter()
                .find(|f| f.to_string() == key)
                .ok_or(TxtError::UnknownField { index })?;

            let parsed_field = parsed_fields
                .iter_mut()
                .find(|f| f.0 == field)
                .ok_or(TxtError::Unknown)?;
            if parsed_field.1 {
                return Err(TxtError::FieldAlreadyExists { index, field });
            }
            parsed_field.1 = true;

            let parse_col_u64 = |field: Field| {
                value.parse::<u64>().map_err(|_| TxtError::InvalidField { index, field })
            };
            let parse_col_i64 = |field: Field| {
                value.parse::<i64>().map_err(|_| TxtError::InvalidField { index, field })
            };

            match field {
                Field::TxId => {
                    transaction.tx_id = parse_col_u64(field)?;
                }
                Field::TxType => {
                    transaction.tx_type = value
                        .parse::<TxType>()
                        .map_err(|_| TxtError::InvalidField { index, field })?;
                }
                Field::FromUserId => {
                    transaction.from_user_id = parse_col_u64(field)?;
                }
                Field::ToUserId => {
                    transaction.to_user_id = parse_col_u64(field)?;
                }
                Field::Amount => {
                    transaction.amount = parse_col_u64(field)?;
                }
                Field::Timestamp => {
                    transaction.timestamp = parse_col_i64(field)?;
                }
                Field::Status => {
                    transaction.status = value
                        .parse::<Status>()
                        .map_err(|_| TxtError::InvalidField { index, field })?;
                }
                Field::Description => {
                    transaction.description = description_trim(value).map_err(
                        |_| TxtError::InvalidField { index, field }
                    )?;
                }
            }
        }

        Ok(transactions)
    }

    fn write_to<W: Write>(
        writer: &mut W,
        transactions: &Vec<Transaction>
    ) -> Result<(), WriteError> {
        for t in transactions {
            let transtaction_record = Field::get_all()
                .map(|field| {
                    let value = t.get_value(&field);
                    let value = if field == Field::Description {
                        format!("\"{value}\"")
                    } else {
                        value
                    };
                    format!("{}: {}", field, value)
                })
                .join("\n");
            write!(writer, "{transtaction_record}\n\n").map_err(|_| WriteError::Write)?;
        }
        writer.flush().map_err(|_| WriteError::Write)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests_from_read {
    use std::io::Cursor;

    use super::*;

    fn get_cursor(lines: Vec<&str>) -> Cursor<String> {
        let data = lines.join("\n");
        let cursor = Cursor::new(data);
        cursor
    }

    #[test]
    fn test_success_from_read() {
        let mut cursor = get_cursor(
            vec![
                "TX_ID: 0",
                "TX_TYPE: DEPOSIT",
                "FROM_USER_ID: 0",
                "TO_USER_ID: 1",
                "AMOUNT: 100",
                "TIMESTAMP: 1633036860000",
                "STATUS: SUCCESS",
                "DESCRIPTION: \"Test 1\""
            ]
        );
        let result = TxtParser::from_read(&mut cursor).unwrap();
        assert_eq!(result, [
            Transaction {
                tx_id: 0,
                tx_type: TxType::Deposit,
                from_user_id: 0,
                to_user_id: 1,
                amount: 100,
                timestamp: 1633036860000,
                status: Status::Success,
                description: "Test 1".to_string(),
            },
        ]);
    }

    #[test]
    fn test_error_line_format() {
        let mut cursor = get_cursor(vec!["Test"]);
        let result = TxtParser::from_read(&mut cursor).unwrap_err();
        assert_eq!(result.to_string(), "Некорректный формат в строке 0");
    }

    #[test]
    fn test_error_unknown_field() {
        let mut cursor = get_cursor(vec!["UNKNOWN_FIELD: 1"]);
        let result = TxtParser::from_read(&mut cursor).unwrap_err();
        assert_eq!(result.to_string(), "Неизвестное поле в строке 0");
    }

    #[test]
    fn test_error_field_already_exists() {
        let mut cursor = get_cursor(vec!["TX_ID: 1", "AMOUNT: 1", "TX_ID: 2"]);
        let result = TxtParser::from_read(&mut cursor).unwrap_err();
        assert_eq!(result.to_string(), "Повторное чтение поле TX_ID в строке 2");
    }

    #[test]
    fn test_error_missing_field() {
        let mut cursor = get_cursor(
            vec![
                "TX_TYPE: DEPOSIT",
                "TO_USER_ID: 9223372036854775807",
                "FROM_USER_ID: 0",
                "TIMESTAMP: 1633036860000",
                "DESCRIPTION: \"Test\"",
                "TX_ID: 1000000000000000",
                "AMOUNT: 100"
                // "STATUS: FAILURE"
            ]
        );
        let result = TxtParser::from_read(&mut cursor).unwrap_err();
        assert_eq!(result.to_string(), "Отсутствует поле STATUS в записи на строке 7");
    }

    #[test]
    fn test_error_invalid_field() {
        let mut cursor = get_cursor(
            vec![
                "TX_ID: !",
                "TX_TYPE: DEPOSIT",
                "FROM_USER_ID: 0",
                "TO_USER_ID: 1",
                "AMOUNT: 100",
                "TIMESTAMP: 1633036860000",
                "STATUS: SUCCESS",
                "DESCRIPTION: \"Test 1\""
            ]
        );
        let result = TxtParser::from_read(&mut cursor).unwrap_err();
        assert_eq!(result.to_string(), "Ошибка парсинга поля TX_ID в строке 0");

        let mut cursor = get_cursor(
            vec![
                "TX_ID: 0",
                "TX_TYPE: !",
                "FROM_USER_ID: 0",
                "TO_USER_ID: 1",
                "AMOUNT: 100",
                "TIMESTAMP: 1633036860000",
                "STATUS: SUCCESS",
                "DESCRIPTION: \"Test 1\""
            ]
        );
        let result = TxtParser::from_read(&mut cursor).unwrap_err();
        assert_eq!(result.to_string(), "Ошибка парсинга поля TX_TYPE в строке 1");

        let mut cursor = get_cursor(
            vec![
                "TX_ID: 0",
                "TX_TYPE: DEPOSIT",
                "FROM_USER_ID: !",
                "TO_USER_ID: 1",
                "AMOUNT: 100",
                "TIMESTAMP: 1633036860000",
                "STATUS: SUCCESS",
                "DESCRIPTION: \"Test 1\""
            ]
        );
        let result = TxtParser::from_read(&mut cursor).unwrap_err();
        assert_eq!(result.to_string(), "Ошибка парсинга поля FROM_USER_ID в строке 2");

        let mut cursor = get_cursor(
            vec![
                "TX_ID: 0",
                "TX_TYPE: DEPOSIT",
                "FROM_USER_ID: 0",
                "TO_USER_ID: !",
                "AMOUNT: 100",
                "TIMESTAMP: 1633036860000",
                "STATUS: SUCCESS",
                "DESCRIPTION: \"Test 1\""
            ]
        );
        let result = TxtParser::from_read(&mut cursor).unwrap_err();
        assert_eq!(result.to_string(), "Ошибка парсинга поля TO_USER_ID в строке 3");

        let mut cursor = get_cursor(
            vec![
                "TX_ID: 0",
                "TX_TYPE: DEPOSIT",
                "FROM_USER_ID: 0",
                "TO_USER_ID: 1",
                "AMOUNT: !",
                "TIMESTAMP: 1633036860000",
                "STATUS: SUCCESS",
                "DESCRIPTION: \"Test 1\""
            ]
        );
        let result = TxtParser::from_read(&mut cursor).unwrap_err();
        assert_eq!(result.to_string(), "Ошибка парсинга поля AMOUNT в строке 4");

        let mut cursor = get_cursor(
            vec![
                "TX_ID: 0",
                "TX_TYPE: DEPOSIT",
                "FROM_USER_ID: 0",
                "TO_USER_ID: 1",
                "AMOUNT: 100",
                "TIMESTAMP: !",
                "STATUS: SUCCESS",
                "DESCRIPTION: \"Test 1\""
            ]
        );
        let result = TxtParser::from_read(&mut cursor).unwrap_err();
        assert_eq!(result.to_string(), "Ошибка парсинга поля TIMESTAMP в строке 5");

        let mut cursor = get_cursor(
            vec![
                "TX_ID: 0",
                "TX_TYPE: DEPOSIT",
                "FROM_USER_ID: 0",
                "TO_USER_ID: 1",
                "AMOUNT: 100",
                "TIMESTAMP: 1633036860000",
                "STATUS: !",
                "DESCRIPTION: \"Test 1\""
            ]
        );
        let result = TxtParser::from_read(&mut cursor).unwrap_err();
        assert_eq!(result.to_string(), "Ошибка парсинга поля STATUS в строке 6");

        let mut cursor = get_cursor(
            vec![
                "TX_ID: 0",
                "TX_TYPE: DEPOSIT",
                "FROM_USER_ID: 0",
                "TO_USER_ID: 1",
                "AMOUNT: 100",
                "TIMESTAMP: 1633036860000",
                "STATUS: SUCCESS",
                "DESCRIPTION: Test 1"
            ]
        );
        let result = TxtParser::from_read(&mut cursor).unwrap_err();
        assert_eq!(result.to_string(), "Ошибка парсинга поля DESCRIPTION в строке 7");
    }
}

#[cfg(test)]
mod tests_write_to {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_success_write_to() {
        let transactions: Vec<Transaction> = vec![
            Transaction {
                tx_id: 1,
                tx_type: TxType::Deposit,
                from_user_id: 0,
                to_user_id: 1,
                amount: 1000,
                timestamp: 1633036860000,
                status: Status::Success,
                description: "record 1".to_string(),
            },
            Transaction {
                tx_id: 2,
                tx_type: TxType::Transfer,
                from_user_id: 1,
                to_user_id: 2,
                amount: 1111,
                timestamp: 1633036860000,
                status: Status::Failure,
                description: "record 2".to_string(),
            }
        ];
        let mut cursor = Cursor::new(Vec::new());
        TxtParser::write_to(&mut cursor, &transactions).unwrap();
        let mut result = String::new();
        cursor.set_position(0);
        let _ = cursor.read_to_string(&mut result);
        assert_eq!(
            result,
            [
                "TX_ID: 1",
                "TX_TYPE: DEPOSIT",
                "FROM_USER_ID: 0",
                "TO_USER_ID: 1",
                "AMOUNT: 1000",
                "TIMESTAMP: 1633036860000",
                "STATUS: SUCCESS",
                "DESCRIPTION: \"record 1\"",
                "",
                "TX_ID: 2",
                "TX_TYPE: TRANSFER",
                "FROM_USER_ID: 1",
                "TO_USER_ID: 2",
                "AMOUNT: 1111",
                "TIMESTAMP: 1633036860000",
                "STATUS: FAILURE",
                "DESCRIPTION: \"record 2\"",
                "",
            ]
                .map(|l| format!("{l}\n"))
                .join("")
        );
    }
}
