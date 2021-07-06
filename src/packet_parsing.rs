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
    use bitbuffer::{bit_buffer::BitBuffer, readable_buf::ReadableBuf};

    #[test]
    fn test_try_parse_succeeds() {
        let mut buf = BitBuffer::new(vec![0b1_0_000111, 0x42]);
        assert_eq!(
            try_parse_field("field", || buf.read_bit_as_bool()).unwrap(),
            true
        );
    }

    #[test]
    fn test_try_parse_fails() {
        let mut buf = BitBuffer::new(Vec::new());
        let result = try_parse_field("field", || buf.read_bit_as_bool());
        assert!(result.is_err());
    }

    #[test]
    fn test_try_parse_multiple_calls() {
        let mut buf = BitBuffer::new(vec![0b1_0_000111, 0x42]);
        let result = try_parse_field("field", || {
            buf.read_bit_as_bool()?;
            buf.read_bit()?;
            buf.read_bits_as_u8(6)
        });
        assert!(result.is_ok());
    }
}
