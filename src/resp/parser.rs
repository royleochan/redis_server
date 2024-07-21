use bytes::{Bytes, BytesMut};
use memchr::memchr2;

use super::data::{RESPDataType, RESPError, RESPResult, CR, NEW_LINE};

/// Find index of carriage return in buffer.
fn find_carraige_return(buffer: &[u8]) -> Option<usize> {
    let end = memchr2(CR, NEW_LINE, buffer)?;
    if *buffer.get(end + 1)? != NEW_LINE {
        return None;
    }
    Some(end)
}

/// Get a word as slice of bytes from buffer, starting at `pos`.
fn parse_word(buffer: &BytesMut, pos: usize) -> Option<(usize, &[u8])> {
    if pos > buffer.len() {
        return None;
    }

    let index = find_carraige_return(&buffer[pos..]);

    match index {
        Some(end_index) => Some((pos + end_index + 2, &buffer[pos..pos + end_index])),
        None => None,
    }
}

/// Get simple string RESPResult from buffer, starting at `pos`.
pub fn to_simple_string(buffer: &BytesMut, pos: usize) -> RESPResult {
    match parse_word(buffer, pos) {
        Some((pos, slice)) => Ok(Some((
            pos,
            RESPDataType::SimpleString(Bytes::copy_from_slice(slice)),
        ))),
        None => Ok(None),
    }
}

/// Get error RESPResult from buffer, starting at `pos`.
pub fn to_error(buffer: &BytesMut, pos: usize) -> RESPResult {
    match parse_word(buffer, pos) {
        Some((pos, slice)) => Ok(Some((
            pos,
            RESPDataType::Error(Bytes::copy_from_slice(slice)),
        ))),
        None => Ok(None),
    }
}

/// Get int RESPResult from buffer, starting at `pos`.
pub fn to_int(buffer: &BytesMut, pos: usize) -> RESPResult {
    match parse_word(buffer, pos) {
        Some((pos, slice)) => {
            let s = std::str::from_utf8(slice).map_err(|_| RESPError::IntParseFailure)?;
            let i = s.parse::<i64>().map_err(|_| RESPError::IntParseFailure)?;
            Ok(Some((pos, RESPDataType::Integer(i))))
        }
        None => Ok(None),
    }
}

/// Get bulk string RESPResult from buffer, starting at `pos`.
pub fn to_bulk_string(buffer: &BytesMut, pos: usize) -> RESPResult {
    match to_int(buffer, pos)? {
        Some((pos, RESPDataType::Integer(-1))) => Ok(Some((pos, RESPDataType::NullBulkString))),
        Some((pos, res)) => match res {
            RESPDataType::Integer(size) => {
                if size >= 0 {
                    let total_size = pos + size as usize;
                    if buffer.len() < total_size + 2 {
                        Ok(None)
                    } else {
                        match parse_word(buffer, pos) {
                            Some((_, slice)) => Ok(Some((
                                total_size + 2,
                                RESPDataType::BulkString(Bytes::copy_from_slice(slice)),
                            ))),
                            None => Ok(None),
                        }
                    }
                } else {
                    Err(RESPError::NegativeBulkStringSize)
                }
            }
            _ => Ok(None),
        },
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use bytes::BufMut;

    use super::*;

    #[test]
    fn test_find_carraige_return() {
        let mut buf = BytesMut::with_capacity(20);
        buf.put(&b"hello world\r\n"[..]);
        assert_eq!(find_carraige_return(&buf).unwrap(), 11);
    }

    #[test]
    fn test_find_carraige_return_none() {
        let mut buf = BytesMut::with_capacity(20);
        buf.put(&b"hello world"[..]);
        assert_eq!(find_carraige_return(&buf), None);
    }

    #[test]
    fn test_parse_word_from_start() {
        let mut buf = BytesMut::with_capacity(20);
        buf.put(&b"hello world\r\n"[..]);
        let result = parse_word(&buf, 0);
        assert_eq!(result, Some((13, &b"hello world"[..])));
    }

    #[test]
    fn test_parse_word_from_middle() {
        let mut buf = BytesMut::with_capacity(20);
        buf.put(&b"hello\r\nworld\r\n"[..]);
        let result = parse_word(&buf, 7);
        assert_eq!(result, Some((14, &b"world"[..])));
    }

    #[test]
    fn test_to_simple_string() {
        let mut buf = BytesMut::with_capacity(20);
        buf.put(&b"OK\r\n"[..]);
        let result = to_simple_string(&buf, 0);
        assert_eq!(
            result.unwrap().unwrap(),
            (4 as usize, RESPDataType::SimpleString(Bytes::from("OK")))
        );
    }

    #[test]
    fn test_to_error() {
        let mut buf = BytesMut::with_capacity(20);
        buf.put(&b"error\r\n"[..]);
        let result = to_error(&buf, 0);
        assert_eq!(
            result.unwrap().unwrap(),
            (7 as usize, RESPDataType::Error(Bytes::from("error")))
        );
    }

    #[test]
    fn test_to_int() {
        let mut buf = BytesMut::with_capacity(20);
        buf.put(&b"64\r\n"[..]);
        let result = to_int(&buf, 0);
        assert_eq!(
            result.unwrap().unwrap(),
            (4 as usize, RESPDataType::Integer(64))
        );
    }

    #[test]
    fn test_bulk_str_null() {
        let mut buf = BytesMut::with_capacity(20);
        buf.put(&b"-1\r\n"[..]);
        let result = to_bulk_string(&buf, 0);
        assert_eq!(
            result.unwrap().unwrap(),
            (4 as usize, RESPDataType::NullBulkString)
        );
    }

    #[test]
    fn test_bulk_str_empty() {
        let mut buf = BytesMut::with_capacity(20);
        buf.put(&b"0\r\n\r\n"[..]);
        let result = to_bulk_string(&buf, 0);
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_bulk_str_normal() {
        let mut buf = BytesMut::with_capacity(20);
        buf.put(&b"5\r\nhello\r\n"[..]);
        let result = to_bulk_string(&buf, 0);
        assert_eq!(
            result.unwrap().unwrap(),
            (10 as usize, RESPDataType::BulkString(Bytes::from("hello")))
        );
    }
}
