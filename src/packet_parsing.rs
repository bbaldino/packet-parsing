use std::fmt::Display;

use crate::error::{wrap, wrap2, RequireEqualError, ValidationError, ValidationResult};

pub fn try_parse_field<T, F: FnOnce() -> Result<T, Box<dyn std::error::Error>>>(
    field_name: &str,
    block: F,
) -> Result<T, Box<dyn std::error::Error>> {
    match block() {
        Ok(v) => Ok(v),
        Err(e) => Err(wrap2(field_name, e)),
    }
}

//TODO: it would be cool to write a bunch of convenience functions like:
//require_value(2) - for when we expect it to be a specific value
//require_within_range(<range>) - for when we expect it to be within a specific range
//etc... for whatever other validation we expect
pub trait Validatable<T> {
    fn validate<F: FnOnce(&T) -> ValidationResult>(
        self,
        validator: F,
    ) -> Result<T, Box<dyn std::error::Error>>;
}

impl<T> Validatable<T> for T
where
    T: Eq,
{
    fn validate<F: FnOnce(&T) -> ValidationResult>(
        self,
        validator: F,
    ) -> Result<T, Box<dyn std::error::Error>> {
        match validator(&self) {
            Ok(_) => Ok(self),
            Err(e) => Err(e.into()),
        }
    }
}

pub trait RequireEqual<T> {
    fn require_value(self, expected: T) -> Result<T, Box<dyn std::error::Error>>;
}

impl<T> RequireEqual<T> for T
where
    T: Eq + Display,
{
    fn require_value(self, expected: T) -> Result<T, Box<dyn std::error::Error>> {
        if self == expected {
            Ok(self)
        } else {
            Err(RequireEqualError(format!(
                "A value of {} was required, got value {}",
                expected, self
            ))
            .into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ValidationError;
    use bitbuffer::{bit_buffer::BitBuffer, readable_buf::ReadableBuf};

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

        //let x: Box<dyn std::error::Error> = buf.read_bit().map_err(Box::new).unwrap_err();
        //let y: Result<Bit, Box<dyn std::error::Error>> = Err((Foo {}).into());
        ////buf.read_bit().map_err(|e| Box::new(Foo {}.into()));
        //let y: Result<Bit, Box<dyn std::error::Error>> = buf.read_bit().map_err(|e| e.into());

        // This works: But we have to do the '?' and the wrap it with Ok() in order to Box
        // the error type (otherwise it complains about having some specific error type and not
        // a Box<dyn Error>.  And unfortunately even doing map_err(Box::new) doesn't work.
        // could the signature on the block function in try_parse_field be changed in such a way
        // that that function could do the boilerplate work?
        //if let Err(e) = try_parse_field("my new field", || buf.read_u16()) {
        //    println!("{}", e);
        //}
        if let Err(e) = parse_header(&mut buf) {
            println!("{}", e);
        }
    }
}
