pub mod parsers;

/// Magic bytes identifying a YPBankBin record header: "YPBN"
pub const MAGIC: [u8; 4] = [0x59, 0x50, 0x42, 0x4E];

/// Header size in bytes (MAGIC + RECORD_SIZE)
pub const HEADER_SIZE: usize = 8;

/// Minimum body size in bytes (fixed fields without description)
pub const MIN_BODY_SIZE: usize = 46;