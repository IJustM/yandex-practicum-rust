use std::{ io::{ Read, Write } };

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
        let mut fields_data: [(Field, Option<String>); 8] = Field::get_all().map(|field| (
            field,
            None,
        ));

        for (index, line) in content.lines().enumerate() {
            if line.starts_with("#") {
                continue;
            }

            if line.is_empty() {
                let mut get_value_and_clean = |field: Field| -> Result<String, TxtError> {
                    let data = fields_data
                        .iter_mut()
                        .find(|f| f.0 == field)
                        .ok_or(TxtError::Unknown)?;
                    let value = data.1.clone().ok_or(TxtError::MissingField { index, field })?;
                    data.1 = None;
                    Ok(value)
                };

                let parse_col_u64 = |value: String, field: Field| {
                    value.parse::<u64>().map_err(|_| TxtError::ParseField {
                        index,
                        field: field.clone(),
                    })
                };
                let parse_col_i64 = |value: String, field: Field| {
                    value.parse::<i64>().map_err(|_| TxtError::ParseField {
                        index,
                        field: field.clone(),
                    })
                };

                transactions.push(Transaction {
                    tx_id: parse_col_u64(get_value_and_clean(Field::TxId)?, Field::TxId)?,
                    tx_type: get_value_and_clean(Field::TxType)?
                        .parse::<TxType>()
                        .map_err(|_| TxtError::ParseField {
                            index,
                            field: Field::TxType,
                        })?,
                    from_user_id: parse_col_u64(
                        get_value_and_clean(Field::FromUserId)?,
                        Field::FromUserId
                    )?,
                    to_user_id: parse_col_u64(
                        get_value_and_clean(Field::ToUserId)?,
                        Field::ToUserId
                    )?,
                    amount: parse_col_u64(get_value_and_clean(Field::Amount)?, Field::Amount)?,
                    timestamp: parse_col_i64(
                        get_value_and_clean(Field::Timestamp)?,
                        Field::Timestamp
                    )?,
                    status: get_value_and_clean(Field::Status)?
                        .parse::<TxStatus>()
                        .map_err(|_| TxtError::ParseField {
                            index,
                            field: Field::Status,
                        })?,
                    description: get_value_and_clean(Field::Description)?,
                });
                continue;
            }

            let field_data = fields_data
                .iter_mut()
                .find(|f| line.starts_with(&format!("{}: ", f.0)))
                .ok_or(TxtError::UnknownField { index })?;
            if field_data.1 == None {
                field_data.1 = Some(line.replace(&format!("{}: ", field_data.0), ""));
            } else {
                return Err(TxtError::FieldAlreadyExists {
                    index,
                    field: field_data.0.clone(),
                });
            }
        }

        Ok(transactions)
    }

    fn write_to<W: Write>(
        transactions: &Vec<Transaction>,
        writer: &mut W
    ) -> Result<(), WriteError> {
        for t in transactions {
            let line = Field::get_all()
                .map(|field| format!("{}: {}", field, t.get_value(&field)))
                .join("\n");
            writeln!(writer, "{}\n", line).map_err(|_| WriteError::Write)?;
        }
        writer.flush().map_err(|_| WriteError::Write)?;
        Ok(())
    }
}
