
use std::str::FromStr;

#[derive(Debug)]
pub enum TransactionType {
    DEPOSIT,
    WITHDRAWAL,
    TRANSFER,
}

impl FromStr for TransactionType {
    type Err = ();

    fn from_str(input: &str) -> Result<TransactionType, Self::Err> {
        match input.to_uppercase().as_str() {
            "DEPOSIT" => Ok(TransactionType::DEPOSIT),
            "WITHDRAWAL" => Ok(TransactionType::WITHDRAWAL),
            "TRANSFER" => Ok(TransactionType::TRANSFER),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum Status {
    PENDING,
    SUCCESS,
    FAILURE,
}

impl FromStr for Status {
    type Err = ();

    fn from_str(input: &str) -> Result<Status, Self::Err> {
        match input.to_uppercase().as_str() {
            "PENDING" => Ok(Status::PENDING),
            "SUCCESS" => Ok(Status::SUCCESS),
            "FAILURE" => Ok(Status::FAILURE),
            _ => Err(()),
        }
    }
}