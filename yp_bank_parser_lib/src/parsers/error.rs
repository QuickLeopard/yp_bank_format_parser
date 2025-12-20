#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("UTF-8 decoding error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("Try From Slice error: {0}")]
    TryFromSliceError(#[from] std::array::TryFromSliceError),
    #[error("Parse Error: {0}")]    
    ParseError(String),    
    #[error("Wrong CSV Header: {0}")]
    WrongCsvHeader(String),
    #[error("Unsupported fromat: {0}")]
    UnsupportedFormat(String),
    #[error("Missing TxId")]
    MissingTxId,
    #[error("Missing From User Id")]
    MissingFromUserId,
    #[error("Missing To User Id")]
    MissingToUserId,
    #[error("Missing Amount")]
    MissingAmount,
    #[error("Missing Timestamp")]
    MissingTimestamp,
    #[error("Missing Status")]
    MissingStatus,
    #[error("Missing Transaction Type")]
    MissingTransactionType,
    #[error("Missing Description")]
    MissingDescription,
    #[error("Wrong Transaction Type: {0}")]
    WrongTransactionType(u8),
    #[error("Wrong status Type: {0}")]
    WrongStatusType(u8),
    #[error("Unexpected EOF: expected {expected} bytes, got {actual} bytes")]
    UnexpectedEof { expected: usize, actual: usize },
    #[error("Description length overflow: desc_len = {desc_len}, remaining bytes = {remaining}")]
    DescriptionOverflow { desc_len: u32, remaining: usize },
    #[error("Invalid magic bytes: expected YPBN (0x59504E42), got {0:02X?}")]
    InvalidMagic([u8; 4]),
    #[error("Record size too small: {0} bytes (minimum: {1})")]
    RecordTooSmall(u32, usize),
    #[error("Record size too large: {0} bytes (maximum: {1})")]
    RecordTooLarge(u32, usize),   
}
