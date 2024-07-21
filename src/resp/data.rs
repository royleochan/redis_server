use bytes::Bytes;

pub const CR: u8 = b'\r';
pub const NEW_LINE: u8 = b'\n';

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RESPDataType {
    SimpleString(Bytes),
    Error(Bytes),
    Integer(i64),
    BulkString(Bytes),
    Array(Vec<RESPDataType>),
}

#[derive(Debug)]
pub enum RESPError {
    UnknownStartingByte,
}

pub type RESPResult = Result<Option<(usize, RESPDataType)>, RESPError>;
