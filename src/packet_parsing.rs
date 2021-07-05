use std::fmt::Display;

use crate::error::{PacketParseError, RequireEqualError, ValidationResult};

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

//TODO: it would be cool to write a bunch of convenience functions like:
//require_value(2) - for when we expect it to be a specific value
//require_within_range(<range>) - for when we expect it to be within a specific range
//etc... for whatever other validation we expect

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        error::ValidationError,
        validators::{RequireEqual, Validatable},
    };
    use bitbuffer::{bit_buffer::BitBuffer, readable_buf::ReadableBuf};

    #[test]
    fn test_try_parse_succeeds() {
        let mut buf = BitBuffer::new(vec![0b1_0_000111, 0x42]);
        assert_eq!(
            try_parse_field("field", || buf.read_bit_as_bool()).unwrap(),
            true
        );
    }

    fn validate_field(value: &u16) -> ValidationResult {
        match value {
            0..=5 => Ok(()),
            v @ _ => Err(ValidationError(format!(
                "Expected value between 0 and 5, got {}",
                v
            ))),
        }
    }

    fn validate_version(value: &u8) -> ValidationResult {
        match value {
            2 => Ok(()),
            v @ _ => Err(ValidationError(format!("Expected version=2, got {}", v))),
        }
    }

    fn validate_packet_type(value: &u8) -> ValidationResult {
        match value {
            90..=120 => Ok(()),
            v @ _ => Err(ValidationError(format!(
                "Expected packet type between 90 and 120, got {}",
                v
            ))),
        }
    }

    struct Header {
        version: u8,
        has_padding: bool,
        report_count: u8,
        packet_type: u8,
    }

    fn parse_header(buf: &mut dyn ReadableBuf) -> Result<Header, Box<dyn std::error::Error>> {
        try_parse_field("header", || {
            Ok(Header {
                version: try_parse_field("version", || buf.read_bits_as_u8(2)?.require_value(2))?,
                has_padding: try_parse_field("has_padding", || {
                    buf.read_bit_as_bool().map_err(Into::into)
                })?,
                report_count: try_parse_field("report count", || {
                    buf.read_bits_as_u8(5).map_err(Into::into)
                })?,
                packet_type: try_parse_field("packet type", || {
                    buf.read_u8()?.validate(validate_packet_type)
                })?,
            })
        })
    }

    #[test]
    fn test() {
        let mut buf = BitBuffer::new(vec![0b10_0_00000, 0b11111111]);

        if let Err(e) = parse_header(&mut buf) {
            println!("{}", e);
        }
    }
}
