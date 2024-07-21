use bytes::{Bytes, BytesMut};
use memchr::memchr2;

use super::data::{RESPDataType, RESPResult, CR, NEW_LINE};

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

// Get simple string RESPResult from buffer, starting at `pos`.
pub fn to_simple_string(buffer: &BytesMut, pos: usize) -> RESPResult {
    match parse_word(buffer, pos) {
        Some((pos, slice)) => Ok(Some((
            pos,
            RESPDataType::SimpleString(Bytes::copy_from_slice(slice)),
        ))),
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
}
