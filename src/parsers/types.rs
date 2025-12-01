use std::str::FromStr;

#[derive(Debug)]
pub struct YPBankCsvRecord {
    pub tx_id: u64,
    pub tx_type: TransactionType,
    pub from_user_id: u64,
    pub to_user_id: u64,
    pub amount: u64,
    pub timestamp: String,
    pub status: Status,
    pub description: String,
}

#[derive(Debug)]
pub enum TransactionType {
    DEPOSIT,
    WITHDRAWAL,
    TRANSFER,
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

#[derive(Debug)]
pub enum Status {
    PENDING,
    SUCCESS,
    FAILURE,
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
