#[derive(Default)]
pub struct RespSerializer;

impl RespSerializer {
    pub fn serialize_ss(self, result: &str) -> String {
        format!("{}{}{}{}", "+", result, "\r", "\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_ss() {
        let resp_serializer = RespSerializer::default();
        assert_eq!(resp_serializer.serialize_ss("result"), "+result\r\n")
    }
}
