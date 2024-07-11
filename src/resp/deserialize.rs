fn deserialize(target: &str) -> &str {
    ""
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_empty() {
        assert_eq!(deserialize(""), "");
    }
}
