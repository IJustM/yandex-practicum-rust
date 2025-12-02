use std::{ io::{ Read, Write } };

use crate::{
    Field,
    Parser,
    Status,
    Transaction,
    TxType,
    errors::WriteError,
    parsers::{ bin::error::BinError, utils::description_trim },
};

pub struct BinParser;

const MAGIC: &[u8; 4] = b"YPBN";
const RECORD_SIZE_WITHOUT_DESC: u32 = 8 + 1 + 8 + 8 + 8 + 8 + 1 + 4;

impl Parser for BinParser {
    type Error = BinError;

    fn from_read<R: Read>(r: &mut R) -> Result<Vec<Transaction>, BinError> {
        let mut data: Vec<u8> = Vec::new();
        r.read_to_end(&mut data).map_err(|_| BinError::Read)?;
        let length = data.len();

        let mut transactions: Vec<Transaction> = Vec::new();

        let mut offset = 0;
        let mut record_index = 0;

        while offset < length {
            let mut take = |n: usize| -> &[u8] {
                let start = offset;
                let end = start + n;
                let value = &data[start..end];
                offset += n;
                value
            };

            let get_value_u32 = |value: &[u8]| -> Result<u32, BinError> {
                Ok(u32::from_be_bytes(value.try_into().map_err(|_| BinError::Unknown)?))
            };
            let get_value_u64 = |value: &[u8]| -> Result<u64, BinError> {
                Ok(u64::from_be_bytes(value.try_into().map_err(|_| BinError::Unknown)?))
            };
            let get_value_i32 = |value: &[u8]| -> Result<i32, BinError> {
                Ok(i32::from_be_bytes(value.try_into().map_err(|_| BinError::Unknown)?))
            };
            let get_value_i64 = |value: &[u8]| -> Result<i64, BinError> {
                Ok(i64::from_be_bytes(value.try_into().map_err(|_| BinError::Unknown)?))
            };

            let magic = take(4);
            if magic != MAGIC {
                return Err(BinError::InvalidMagic { index: record_index });
            }
            let _record_size = get_value_u32(take(4)).map_err(|_| BinError::InvalidRecordSize {
                index: record_index,
            })?;

            let tx_id = get_value_u64(take(8)).map_err(|_| BinError::InvalidField {
                index: record_index,
                field: Field::TxId,
            })?;
            let tx_type: TxType = match take(1)[0] {
                0 => TxType::Deposit,
                1 => TxType::Transfer,
                2 => TxType::Withdrawal,
                _ => Err(BinError::InvalidField { index: record_index, field: Field::TxType })?,
            };
            let from_user_id = get_value_u64(take(8)).map_err(|_| BinError::InvalidField {
                index: record_index,
                field: Field::FromUserId,
            })?;
            let to_user_id = get_value_u64(take(8)).map_err(|_| BinError::InvalidField {
                index: record_index,
                field: Field::ToUserId,
            })?;
            let amount = get_value_u64(take(8)).map_err(|_| BinError::InvalidField {
                index: record_index,
                field: Field::Amount,
            })?;
            let timestamp = get_value_i64(take(8)).map_err(|_| BinError::InvalidField {
                index: record_index,
                field: Field::Timestamp,
            })?;
            let status: Status = match take(1)[0] {
                0 => Status::Success,
                1 => Status::Failure,
                2 => Status::Pending,
                _ => Err(BinError::InvalidField { index: record_index, field: Field::Status })?,
            };
            let desc_len = get_value_i32(take(4)).map_err(|_| BinError::InvalidDescLen {
                index: record_index,
            })?;
            let description = str
                ::from_utf8(take(desc_len as usize))
                .map_err(|_| BinError::InvalidField {
                    index: record_index,
                    field: Field::Description,
                })?
                .to_string();
            let description = description_trim(&description).map_err(|_| BinError::InvalidField {
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

    fn write_to<W: Write>(
        writer: &mut W,
        transactions: &Vec<Transaction>
    ) -> Result<(), WriteError> {
        for t in transactions {
            let mut data: Vec<u8> = Vec::new();
            data.extend_from_slice(MAGIC);

            let description = t.description.as_bytes();
            let desc_len = description.len() as u32;

            data.extend_from_slice(
                &((RECORD_SIZE_WITHOUT_DESC + desc_len).to_be_bytes() as [u8; 4])
            );
            data.extend_from_slice(&(t.tx_id.to_be_bytes() as [u8; 8]));
            data.extend_from_slice(
                &[
                    match t.tx_type {
                        TxType::Deposit => 0x0,
                        TxType::Transfer => 0x1,
                        TxType::Withdrawal => 0x2,
                    },
                ]
            );
            data.extend_from_slice(&(t.from_user_id.to_be_bytes() as [u8; 8]));
            data.extend_from_slice(&(t.to_user_id.to_be_bytes() as [u8; 8]));
            data.extend_from_slice(&(t.amount.to_be_bytes() as [u8; 8]));
            data.extend_from_slice(&(t.timestamp.to_be_bytes() as [u8; 8]));
            data.extend_from_slice(
                &[
                    match t.status {
                        Status::Success => 0x0,
                        Status::Failure => 0x1,
                        Status::Pending => 0x2,
                    },
                ]
            );
            data.extend_from_slice(&(desc_len.to_be_bytes() as [u8; 4]));
            data.extend_from_slice(description);

            writer.write_all(&data).map_err(|_| WriteError::Write)?;
        }
        writer.flush().map_err(|_| WriteError::Write)?;
        Ok(())
    }
}
