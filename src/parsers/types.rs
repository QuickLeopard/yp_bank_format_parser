use std::str::FromStr;

use crate::parsers::error::ParserError;

#[derive(Debug, Clone)]
pub struct YPBankCsvRecord {
    pub tx_id: u64,
    pub tx_type: TransactionType,
    pub from_user_id: u64,
    pub to_user_id: u64,
    pub amount: i64,
    pub timestamp: String,
    pub status: Status,
    pub description: String,
}

#[derive(Debug, Clone, Copy)]
pub enum TransactionType {
    DEPOSIT = 0,
    TRANSFER = 1,
    WITHDRAWAL = 2,
}

impl FromStr for TransactionType {
    type Err = String;

    fn from_str(input: &str) -> Result<TransactionType, Self::Err> {
        match input.to_uppercase().as_str() {
            "DEPOSIT" => Ok(TransactionType::DEPOSIT),
            "WITHDRAWAL" => Ok(TransactionType::WITHDRAWAL),
            "TRANSFER" => Ok(TransactionType::TRANSFER),
            _ => Err("Wrong transaction type".to_string()),
        }
    }
}

impl TryFrom<u8> for TransactionType {
    type Error = ParserError;
    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TransactionType::DEPOSIT),
            1 => Ok(TransactionType::TRANSFER),
            2 => Ok(TransactionType::WITHDRAWAL),
            _ => Err(ParserError::ParseError(format! ("Wrong Transaction Type: {}", value))),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Status {
    SUCCESS = 0,
    FAILURE = 1,
    PENDING = 2,
}

impl FromStr for Status {
    type Err = String;

    fn from_str(input: &str) -> Result<Status, Self::Err> {
        match input.to_uppercase().as_str() {
            "PENDING" => Ok(Status::PENDING),
            "SUCCESS" => Ok(Status::SUCCESS),
            "FAILURE" => Ok(Status::FAILURE),
            _ => Err("Wrong status".to_string()),
        }
    }
}

impl TryFrom<u8> for Status {
    type Error = ParserError;
    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Status::SUCCESS ),
            1 => Ok(Status::FAILURE),
            2 => Ok(Status::PENDING),
            _ => Err(ParserError::ParseError(format! ("Wrong Status: {}", value))),
        }
    }
}
