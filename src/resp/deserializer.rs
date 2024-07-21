use bytes::{Bytes, BytesMut};

use super::data::{RESPDataType, RESPError, RESPResult};
use super::parser::to_simple_string;

#[derive(Default)]
pub struct RespDeserializer;

impl RespDeserializer {
    fn deserialize_word(self, buffer: &BytesMut, pos: usize) -> RESPResult {
        if buffer.is_empty() {
            return Ok(None);
        }

        match buffer.get(0) {
            Some(b'+') => to_simple_string(buffer, pos + 1),
            _ => Err(RESPError::UnknownStartingByte),
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::BufMut;

    use super::*;

    #[test]
    fn test_deserialize_word() {
        let mut buf = BytesMut::with_capacity(20);
        buf.put(&b"+OK\r\n"[..]);
        let resp_deserializer = RespDeserializer::default();
        // let a = resp_deserializer.deserialize_word(&buf, 0);
        assert_eq!(
            resp_deserializer
                .deserialize_word(&buf, 0)
                .unwrap()
                .unwrap(),
            (5 as usize, RESPDataType::SimpleString(Bytes::from("OK")))
        )
    }
}
