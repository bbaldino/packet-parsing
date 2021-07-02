use thiserror::Error;

use bitbuffer::error::BitBufferError;

use crate::error::ValidationError;
// let version = buf.read_bits_field(2, "version")?.validate(|v| v == 2)?;
// let version = field("version", || buf.read_bits_as_u8(2), |v| v == 2)?;
// // try_parse_field would wrap the inner error and add the context to the log message.  This
// means we wouldn't have specific errors in PacketParseError, it would just wrap a Box<dyn Error>
// let version = try_parse_field("version", || {
//     // This methods just reutn their individual errors (BitBufferError, ValidationError)
//     buf.read_bits_as_u8(2)?.validate(|v| v == 2)?
// });
// Header {
//     version: try_parse_field("version", || buf.read_bits_as_u8(2)?.validate(validate_version))?,
//     has_padding: try_parse_field("has_padding", ||
// }

// This is an error with context: it contains not only the underlying error but the context
// of what was being attempted at the time of the error

#[derive(Error, Debug)]
enum NewPacketParseError {
    #[error("Error parsing field '{field_name}': {error}")]
    FieldParseError {
        field_name: String,
        error: Box<dyn std::error::Error>,
    },
    #[error("Error parsing field group '{field_group_name}': {error}")]
    FieldGroupParseError {
        field_group_name: String,
        error: Box<NewPacketParseError>,
    },
}

// Any sort of error that can happen (buffer-level error, validation error, etc.) uses this.  It
// wraps either a successfully-retrieved value or some error.  It has no context (what was being
// attempted at the time of the error)
type ParseResult<T> = Result<T, Box<dyn std::error::Error>>;

type FieldResult<T> = Result<T, NewPacketParseError>;

fn try_parse_field<T, F: FnOnce() -> ParseResult<T>>(field_name: &str, block: F) -> FieldResult<T> {
    match block() {
        Ok(v) => Ok(v),
        Err(e) => Err(NewPacketParseError::FieldParseError {
            field_name: field_name.to_owned(),
            error: e,
        }),
    }
}

fn fake_read_u8(succeed: bool) -> ParseResult<u8> {
    if succeed {
        Ok(42)
    } else {
        Err(Box::new(BitBufferError::BufferTooShort {
            start_pos: 0,
            num_bytes: 10,
            buffer_size: 5,
        }))
    }
}

fn fake_validate() -> ParseResult<u8> {
    Err(Box::new(ValidationError(
        "input didn't match expected".to_owned(),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_thing() -> FieldResult<u8> {
        try_parse_field("my field", || {
            let res = fake_read_u8(false)?;
            let res = fake_validate()?;
            Ok(res)
        })
    }

    #[test]
    fn test_name() {
        if let Err(e) = parse_thing() {
            println!("{}", e);
        }
    }
}
