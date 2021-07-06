use crate::error::PacketParseError;

pub fn try_parse_field<T, F: FnOnce() -> Result<T, Box<dyn std::error::Error>>>(
    field_name: &str,
    block: F,
) -> Result<T, Box<dyn std::error::Error>> {
    match block() {
        Ok(v) => Ok(v),
        Err(e) => Err(Box::new(PacketParseError {
            field_name: field_name.to_owned(),
            error: e.into(),
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use thiserror::Error;

    #[derive(Error, Debug)]
    #[error("Error: {0}")]
    struct MyError(String);

    fn parse_field_success() -> Result<u8, Box<dyn std::error::Error>> {
        Ok(42)
    }

    fn parse_field_error() -> Result<u8, Box<dyn std::error::Error>> {
        Err(Box::new(MyError("Error".to_owned())))
    }

    #[test]
    fn test_try_parse_succeeds() {
        assert_eq!(
            try_parse_field("field", || parse_field_success()).unwrap(),
            42
        );
    }

    #[test]
    fn test_try_parse_fails() {
        let result = try_parse_field("field", || parse_field_error());
        assert!(result.is_err());
    }

    #[test]
    fn test_try_parse_multiple_calls() {
        let result = try_parse_field("field", || {
            parse_field_success()?;
            parse_field_success()?;
            parse_field_success()
        });
        assert!(result.is_ok());
    }
}
