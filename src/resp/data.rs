use bytes::Bytes;

pub const CR: u8 = b'\r';
pub const NEW_LINE: u8 = b'\n';

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RESPDataType {
    SimpleString(Bytes),
    Error(Bytes),
    Integer(i64),
    BulkString(Bytes),
    NullBulkString,
    Array(Vec<RESPDataType>),
    NullArray,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RESPError {
    UnknownStartingByte,
    IntParseFailure,
    InvalidBulkStringSize,
    InvalidArrayElementSize,
}

pub type RESPResult = Result<Option<(usize, RESPDataType)>, RESPError>;
