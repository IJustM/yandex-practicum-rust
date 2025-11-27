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
        let mut exists: [(Field, Option<String>); 8] = Field::get_all().map(|field| (field, None));

        for (index, line) in content.lines().enumerate() {
            if line.starts_with("#") {
                continue;
            }

            if line.is_empty() {
                let mut transaction = Transaction::default();
                for (field, value) in exists.iter_mut() {
                    match value {
                        Some(v) => {
                            let parse_col_u64 = || {
                                v.parse::<u64>().map_err(|_| TxtError::ParseField {
                                    index,
                                    field: field.clone(),
                                })
                            };
                            let parse_col_i64 = || {
                                v.parse::<i64>().map_err(|_| TxtError::ParseField {
                                    index,
                                    field: field.clone(),
                                })
                            };

                            match field {
                                Field::TxId => {
                                    transaction.tx_id = parse_col_u64()?;
                                }
                                Field::TxType => {
                                    transaction.tx_type = v
                                        .parse::<TxType>()
                                        .map_err(|_| TxtError::ParseField {
                                            index,
                                            field: Field::TxType,
                                        })?;
                                }
                                Field::FromUserId => {
                                    transaction.from_user_id = parse_col_u64()?;
                                }
                                Field::ToUserId => {
                                    transaction.to_user_id = parse_col_u64()?;
                                }
                                Field::Amount => {
                                    transaction.amount = parse_col_u64()?;
                                }
                                Field::Timestamp => {
                                    transaction.timestamp = parse_col_i64()?;
                                }
                                Field::Status => {
                                    transaction.status = v
                                        .parse::<TxStatus>()
                                        .map_err(|_| TxtError::ParseField {
                                            index,
                                            field: Field::TxType,
                                        })?;
                                }
                                Field::Description => {
                                    transaction.description = v.trim_matches('"').to_string();
                                }
                            }
                            *value = None;
                        }
                        None => {
                            return Err(TxtError::MissingField { index, field: field.clone() });
                        }
                    }
                }
                transactions.push(transaction);
                continue;
            }

            match exists.iter_mut().find(|f| line.starts_with(&format!("{}: ", f.0))) {
                Some((field, value)) => {
                    match value {
                        Some(_) => {
                            return Err(TxtError::FieldAlreadyExists {
                                index,
                                field: field.clone(),
                            });
                        }
                        None => {
                            *value = Some(line.replace(&format!("{}: ", field), ""));
                        }
                    }
                }
                None => {
                    return Err(TxtError::UnknownField { index });
                }
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
