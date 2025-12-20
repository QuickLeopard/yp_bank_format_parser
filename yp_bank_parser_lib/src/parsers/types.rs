use std::str::FromStr;

use crate::parsers::error::ParserError;

/// Represents a YPBank transaction record.
#[derive(Debug, Clone, PartialEq)]
pub struct YPBankRecord {
    pub tx_id: u64,
    pub tx_type: TransactionType,
    pub from_user_id: u64,
    pub to_user_id: u64,
    pub amount: i64,
    pub timestamp: u64,
    pub status: Status,
    pub description: String,
}

/// Transaction type enumeration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransactionType {
    Deposit = 0,
    Transfer = 1,
    Withdrawal = 2,
}

impl FromStr for TransactionType {
    type Err = String;

    fn from_str(input: &str) -> Result<TransactionType, Self::Err> {
        match input.to_uppercase().as_str() {
            "DEPOSIT" => Ok(TransactionType::Deposit),
            "WITHDRAWAL" => Ok(TransactionType::Withdrawal),
            "TRANSFER" => Ok(TransactionType::Transfer),
            _ => Err("Wrong transaction type".to_string()),
        }
    }
}

impl TryFrom<u8> for TransactionType {
    type Error = ParserError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TransactionType::Deposit),
            1 => Ok(TransactionType::Transfer),
            2 => Ok(TransactionType::Withdrawal),
            _ => Err(ParserError::ParseError(format!(
                "Wrong Transaction Type: {}",
                value
            ))),
        }
    }
}

impl TransactionType {
    /// Parse a transaction type from a byte value.
    ///
    /// # Arguments
    ///
    /// * `value` - Byte value to parse (0=Deposit, 1=Transfer, 2=Withdrawal)
    ///
    /// # Returns
    ///
    /// Returns a Result containing the TransactionType on success, or ParserError on failure.
    pub fn from_byte(value: u8) -> Result<Self, ParserError> {
        match value {
            0 => Ok(TransactionType::Deposit),
            1 => Ok(TransactionType::Transfer),
            2 => Ok(TransactionType::Withdrawal),
            _ => Err(ParserError::WrongTransactionType(value)),
        }
    }

    /// Convert to byte representation.
    ///
    /// # Returns
    ///
    /// Returns the byte value representing this transaction type.
    pub fn to_byte(self) -> u8 {
        self as u8
    }
}

/// Transaction status enumeration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    Success = 0,
    Failure = 1,
    Pending = 2,
}

impl FromStr for Status {
    type Err = String;

    fn from_str(input: &str) -> Result<Status, Self::Err> {
        match input.to_uppercase().as_str() {
            "PENDING" => Ok(Status::Pending),
            "SUCCESS" => Ok(Status::Success),
            "FAILURE" => Ok(Status::Failure),
            _ => Err("Wrong status".to_string()),
        }
    }
}

impl TryFrom<u8> for Status {
    type Error = ParserError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Status::Success),
            1 => Ok(Status::Failure),
            2 => Ok(Status::Pending),
            _ => Err(ParserError::ParseError(format!("Wrong Status: {}", value))),
        }
    }
}

impl Status {
    /// Parse a status from a byte value.
    ///
    /// # Arguments
    ///
    /// * `value` - Byte value to parse (0=Success, 1=Failure, 2=Pending)
    ///
    /// # Returns
    ///
    /// Returns a Result containing the Status on success, or ParserError on failure.
    pub fn from_byte(value: u8) -> Result<Self, ParserError> {
        match value {
            0 => Ok(Status::Success),
            1 => Ok(Status::Failure),
            2 => Ok(Status::Pending),
            _ => Err(ParserError::WrongStatusType(value)),
        }
    }

    /// Convert to byte representation.
    ///
    /// # Returns
    ///
    /// Returns the byte value representing this status.
    pub fn to_byte(self) -> u8 {
        self as u8
    }
}
