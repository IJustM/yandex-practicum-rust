use std::io::{Read, Write};

use crate::{
    Field, Parser, Status, Transaction, TxType,
    errors::WriteError,
    parsers::{bin::error::BinError, utils::description_trim},
};

/// Парсер для bin формата
pub struct BinParser;

const MAGIC: &[u8; 4] = b"YPBN";
const RECORD_SIZE_WITHOUT_DESC: u32 = 8 + 1 + 8 + 8 + 8 + 8 + 1 + 4;

impl Parser for BinParser {
    type Error = BinError;

    fn from_read<R: Read>(reader: &mut R) -> Result<Vec<Transaction>, BinError> {
        let mut data: Vec<u8> = Vec::new();
        reader.read_to_end(&mut data).map_err(|_| BinError::Read)?;
        let length = data.len();

        let mut transactions: Vec<Transaction> = Vec::new();

        let mut offset = 0;
        let mut record_index = 0;

        while offset < length {
            let mut take = |n: usize| -> Result<&[u8], BinError> {
                let start = offset;
                let end = start + n;
                if end > length {
                    return Err(BinError::InvalidLength {
                        index: record_index,
                    });
                }
                let value = &data[start..end];
                offset += n;
                Ok(value)
            };

            let get_value_u32 = |value: &[u8]| -> Result<u32, BinError> {
                Ok(u32::from_be_bytes(
                    value.try_into().map_err(|_| BinError::Unknown)?,
                ))
            };
            let get_value_u64 = |value: &[u8]| -> Result<u64, BinError> {
                Ok(u64::from_be_bytes(
                    value.try_into().map_err(|_| BinError::Unknown)?,
                ))
            };
            let get_value_i32 = |value: &[u8]| -> Result<i32, BinError> {
                Ok(i32::from_be_bytes(
                    value.try_into().map_err(|_| BinError::Unknown)?,
                ))
            };
            let get_value_i64 = |value: &[u8]| -> Result<i64, BinError> {
                Ok(i64::from_be_bytes(
                    value.try_into().map_err(|_| BinError::Unknown)?,
                ))
            };

            let magic = take(4)?;
            if magic != MAGIC {
                return Err(BinError::InvalidMagic {
                    index: record_index,
                });
            }
            let _record_size =
                get_value_u32(take(4)?).map_err(|_| BinError::InvalidRecordSize {
                    index: record_index,
                })?;

            let tx_id = get_value_u64(take(8)?).map_err(|_| BinError::InvalidField {
                index: record_index,
                field: Field::TxId,
            })?;
            let tx_type: TxType = match take(1)?[0] {
                0 => TxType::Deposit,
                1 => TxType::Transfer,
                2 => TxType::Withdrawal,
                _ => Err(BinError::InvalidField {
                    index: record_index,
                    field: Field::TxType,
                })?,
            };
            let from_user_id = get_value_u64(take(8)?).map_err(|_| BinError::InvalidField {
                index: record_index,
                field: Field::FromUserId,
            })?;
            let to_user_id = get_value_u64(take(8)?).map_err(|_| BinError::InvalidField {
                index: record_index,
                field: Field::ToUserId,
            })?;
            let amount = get_value_u64(take(8)?).map_err(|_| BinError::InvalidField {
                index: record_index,
                field: Field::Amount,
            })?;
            let timestamp = get_value_i64(take(8)?).map_err(|_| BinError::InvalidField {
                index: record_index,
                field: Field::Timestamp,
            })?;
            let status: Status = match take(1)?[0] {
                0 => Status::Success,
                1 => Status::Failure,
                2 => Status::Pending,
                _ => Err(BinError::InvalidField {
                    index: record_index,
                    field: Field::Status,
                })?,
            };
            let desc_len = get_value_i32(take(4)?).map_err(|_| BinError::InvalidDescLen {
                index: record_index,
            })?;
            let description = str::from_utf8(take(desc_len as usize)?)
                .map_err(|_| BinError::InvalidField {
                    index: record_index,
                    field: Field::Description,
                })?
                .to_string();
            let description =
                description_trim(&description).map_err(|_| BinError::InvalidField {
                    index: record_index,
                    field: Field::Description,
                })?;

            record_index += 1;
            transactions.push(Transaction {
                tx_id,
                tx_type,
                from_user_id,
                to_user_id,
                amount,
                timestamp,
                status,
                description,
            });
        }

        Ok(transactions)
    }

    fn write_to<W: Write>(writer: &mut W, transactions: &[Transaction]) -> Result<(), WriteError> {
        for t in transactions {
            let mut data: Vec<u8> = Vec::new();
            data.extend_from_slice(MAGIC);

            let description = format!("\"{}\"", t.description);
            let description = description.as_bytes();
            let desc_len = description.len() as u32;

            data.extend_from_slice(
                &((RECORD_SIZE_WITHOUT_DESC + desc_len).to_be_bytes() as [u8; 4]),
            );
            data.extend_from_slice(&(t.tx_id.to_be_bytes() as [u8; 8]));
            data.extend_from_slice(&[match t.tx_type {
                TxType::Deposit => 0,
                TxType::Transfer => 1,
                TxType::Withdrawal => 2,
            }]);
            data.extend_from_slice(&(t.from_user_id.to_be_bytes() as [u8; 8]));
            data.extend_from_slice(&(t.to_user_id.to_be_bytes() as [u8; 8]));
            data.extend_from_slice(&(t.amount.to_be_bytes() as [u8; 8]));
            data.extend_from_slice(&(t.timestamp.to_be_bytes() as [u8; 8]));
            data.extend_from_slice(&[match t.status {
                Status::Success => 0,
                Status::Failure => 1,
                Status::Pending => 2,
            }]);
            data.extend_from_slice(&(desc_len.to_be_bytes() as [u8; 4]));
            data.extend_from_slice(description);

            writer.write_all(&data).map_err(|_| WriteError::Write)?;
        }
        writer.flush().map_err(|_| WriteError::Write)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests_from_read {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_success_from_read() {
        let mut data: Vec<u8> = Vec::new();

        data.extend_from_slice(&[89, 80, 66, 78]); // MAGIC
        data.extend_from_slice(&[0, 0, 0, 54]); // RECORD_SIZE
        data.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 1]); // ID
        data.extend_from_slice(&[0]); // TX_TYPE
        data.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]); // FROM_USER_ID
        data.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 1]); // TO_USER_ID
        data.extend_from_slice(&[0, 0, 0, 0, 0, 0, 3, 232]); // AMOUNT
        data.extend_from_slice(&[0, 0, 1, 124, 56, 148, 250, 96]); // TIMESTAMP
        data.extend_from_slice(&[0]); // STATUS
        data.extend_from_slice(&[0, 0, 0, 10]); // DESC_LEN
        data.extend_from_slice(&[34, 114, 101, 99, 111, 114, 100, 32, 49, 34]); // DESCRIPTION

        let mut cursor = Cursor::new(data);

        let result = BinParser::from_read(&mut cursor).unwrap();
        assert_eq!(
            result,
            [Transaction {
                tx_id: 1,
                tx_type: TxType::Deposit,
                from_user_id: 0,
                to_user_id: 1,
                amount: 1000,
                timestamp: 1633036860000,
                status: Status::Success,
                description: "record 1".to_string(),
            },]
        );
    }

    #[test]
    fn test_error_invalid_length() {
        let mut data: Vec<u8> = Vec::new();
        data.extend_from_slice(&[0]);

        let mut cursor = Cursor::new(data);

        let result = BinParser::from_read(&mut cursor).unwrap_err();
        assert_eq!(result.to_string(), "Неожиданное завершение записи 0");
    }

    #[test]
    fn test_error_invalid_magic() {
        let mut data: Vec<u8> = Vec::new();
        data.extend_from_slice(&[0, 1, 2, 3]);

        let mut cursor = Cursor::new(data);

        let result = BinParser::from_read(&mut cursor).unwrap_err();
        assert_eq!(result.to_string(), "Некорректный MAGIC в записи 0");
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
            },
        ];
        let mut cursor = Cursor::new(Vec::new());
        BinParser::write_to(&mut cursor, &transactions).unwrap();
        let mut result = Vec::new();
        cursor.set_position(0);
        let _ = cursor.read_to_end(&mut result);

        let mut expected = Vec::new();
        // record 1
        expected.extend_from_slice(&[89, 80, 66, 78]); // MAGIC
        expected.extend_from_slice(&[0, 0, 0, 56]); // RECORD_SIZE
        expected.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 1]); // ID
        expected.extend_from_slice(&[0]); // TX_TYPE
        expected.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]); // FROM_USER_ID
        expected.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 1]); // TO_USER_ID
        expected.extend_from_slice(&[0, 0, 0, 0, 0, 0, 3, 232]); // AMOUNT
        expected.extend_from_slice(&[0, 0, 1, 124, 56, 148, 250, 96]); // TIMESTAMP
        expected.extend_from_slice(&[0]); // STATUS
        expected.extend_from_slice(&[0, 0, 0, 10]); // DESC_LEN
        expected.extend_from_slice(&[34, 114, 101, 99, 111, 114, 100, 32, 49, 34]); // DESCRIPTION

        // record 2
        expected.extend_from_slice(&[89, 80, 66, 78]); // MAGIC
        expected.extend_from_slice(&[0, 0, 0, 56]); // RECORD_SIZE
        expected.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 2]); // ID
        expected.extend_from_slice(&[1]); // TX_TYPE
        expected.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 1]); // FROM_USER_ID
        expected.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 2]); // TO_USER_ID
        expected.extend_from_slice(&[0, 0, 0, 0, 0, 0, 4, 87]); // AMOUNT
        expected.extend_from_slice(&[0, 0, 1, 124, 56, 148, 250, 96]); // TIMESTAMP
        expected.extend_from_slice(&[1]); // STATUS
        expected.extend_from_slice(&[0, 0, 0, 10]); // DESC_LEN
        expected.extend_from_slice(&[34, 114, 101, 99, 111, 114, 100, 32, 50, 34]); // DESCRIPTION

        assert_eq!(result, expected);
    }
}
