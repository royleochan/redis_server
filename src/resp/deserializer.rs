use bytes::{Bytes, BytesMut};

use super::data::{RESPDataType, RESPError, RESPResult};
use super::parser::{to_array, to_bulk_string, to_error, to_int, to_simple_string};

#[derive(Default)]
pub struct RespDeserializer;

impl RespDeserializer {
    pub fn deserialize_word(self, buffer: &BytesMut, pos: usize) -> RESPResult {
        if buffer.is_empty() {
            return Ok(None);
        }

        match buffer.get(pos) {
            Some(b'+') => to_simple_string(buffer, pos + 1),
            Some(b'-') => to_error(buffer, pos + 1),
            Some(b':') => to_int(buffer, pos + 1),
            Some(b'$') => to_bulk_string(buffer, pos + 1),
            Some(b'*') => to_array(buffer, pos + 1),
            _ => Err(RESPError::UnknownStartingByte),
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::BufMut;

    use super::*;

    #[test]
    fn test_deserialize_ss() {
        let mut buf = BytesMut::with_capacity(20);
        buf.put(&b"+OK\r\n"[..]);
        let resp_deserializer = RespDeserializer::default();
        assert_eq!(
            resp_deserializer
                .deserialize_word(&buf, 0)
                .unwrap()
                .unwrap(),
            (5 as usize, RESPDataType::SimpleString(Bytes::from("OK")))
        )
    }

    #[test]
    fn test_deserialize_error() {
        let mut buf = BytesMut::with_capacity(20);
        buf.put(&b"-Error message\r\n"[..]);
        let resp_deserializer = RespDeserializer::default();
        assert_eq!(
            resp_deserializer
                .deserialize_word(&buf, 0)
                .unwrap()
                .unwrap(),
            (
                16 as usize,
                RESPDataType::Error(Bytes::from("Error message"))
            )
        )
    }

    #[test]
    fn test_deserialize_int() {
        let mut buf = BytesMut::with_capacity(20);
        buf.put(&b":1024\r\n"[..]);
        let resp_deserializer = RespDeserializer::default();
        assert_eq!(
            resp_deserializer
                .deserialize_word(&buf, 0)
                .unwrap()
                .unwrap(),
            (7 as usize, RESPDataType::Integer(1024))
        )
    }

    #[test]
    fn test_deserialize_bulk_str() {
        let mut buf = BytesMut::with_capacity(20);
        buf.put(&b"$5\r\nlorem\r\n"[..]);
        let resp_deserializer = RespDeserializer::default();
        assert_eq!(
            resp_deserializer
                .deserialize_word(&buf, 0)
                .unwrap()
                .unwrap(),
            (11 as usize, RESPDataType::BulkString(Bytes::from("lorem")))
        )
    }

    #[test]
    fn test_deserialize_array() {
        let mut buf = BytesMut::with_capacity(20);
        buf.put(&b"*2\r\n$4\r\necho\r\n$11\r\nhello world\r\n"[..]);
        let resp_deserializer = RespDeserializer::default();
        let expected_vec = vec![
            RESPDataType::BulkString(Bytes::from("echo")),
            RESPDataType::BulkString(Bytes::from("hello world")),
        ];
        assert_eq!(
            resp_deserializer
                .deserialize_word(&buf, 0)
                .unwrap()
                .unwrap(),
            (32 as usize, RESPDataType::Array(expected_vec))
        )
    }

    #[test]
    fn test_unknown_starting_byte() {
        let mut buf = BytesMut::with_capacity(20);
        buf.put(&b"@Unknown\r\n"[..]);
        let resp_deserializer = RespDeserializer::default();
        assert_eq!(
            resp_deserializer.deserialize_word(&buf, 0).unwrap_err(),
            RESPError::UnknownStartingByte
        )
    }
}
