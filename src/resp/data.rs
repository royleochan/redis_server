pub const CLRF: &str = "\r\n";
pub const NULL_STR: &str = "$-1\r\n";
pub const NULL_ARR: &str = "-1\r\n";

#[derive(Debug, PartialEq, Eq)]
enum DataType {
    SimpleString,
    Error,
    Integer,
    BulkString,
    Array,
}

impl DataType {
    fn from(character: &str) -> Result<Self, String> {
        match character {
            "+" => Ok(DataType::SimpleString),
            "-" => Ok(DataType::Error),
            ":" => Ok(DataType::Integer),
            "$" => Ok(DataType::BulkString),
            "*" => Ok(DataType::Array),
            _ => Err("Invalid data type".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const() {
        let result = "\r\n";
        assert_eq!(result, CLRF);
    }

    #[test]
    fn test_from_valid() {
        assert_eq!(DataType::from("+").unwrap(), DataType::SimpleString);
        assert_eq!(DataType::from("-").unwrap(), DataType::Error);
        assert_eq!(DataType::from(":").unwrap(), DataType::Integer);
        assert_eq!(DataType::from("$").unwrap(), DataType::BulkString);
        assert_eq!(DataType::from("*").unwrap(), DataType::Array);
    }

    #[test]
    #[should_panic(expected = "Invalid data type")]
    fn test_from_invalid() {
        DataType::from("lorem").unwrap();
    }
}
