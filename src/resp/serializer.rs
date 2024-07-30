#[derive(Default)]
pub struct RespSerializer;

impl RespSerializer {
    pub fn serialize_ss(self, result: &str) -> String {
        format!("{}{}{}{}", "+", result, "\r", "\n")
    }

    pub fn serialize_error(self, result: &str) -> String {
        format!("{}{}{}{}", "-", result, "\r", "\n")
    }

    pub fn serialize_nil(self) -> String {
        return String::from("*-1\r\n");
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

    #[test]
    fn test_serialize_error() {
        let resp_serializer = RespSerializer::default();
        assert_eq!(resp_serializer.serialize_error("error"), "-error\r\n")
    }
}
