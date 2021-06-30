use bitbuffer::bit::Bit;
use bitbuffer::error::BitBufferError;
use bitbuffer::readable_buf::ReadableBuf;
use bitbuffer::bit_buffer::BitBuffer;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PacketParseError {
    #[error("Unable to parse field '{field_name}': {source}")]
    BufferError {
        field_name: String,
        source: BitBufferError
    },
    #[error("Validation failed for field '{field_name}': '{msg}'")]
    ValidationError {
        field_name: String,
        msg: String,
    }
}

pub struct ValidationError(String);

pub type PacketParseResult<T> = Result<T, PacketParseError>;
pub type ValidationResult = Result<(), ValidationError>;

fn read_bit_field(buf: &mut dyn ReadableBuf, field_name: &str) -> PacketParseResult<Bit> {
    match buf.read_bit() {
        Ok(b) => Ok(b),
        Err(e) => Err(PacketParseError::BufferError {
            field_name: field_name.to_owned(),
            source: e
        })
    }
}

fn read_and_validate_bit_field<F: FnOnce(&Bit) -> ValidationResult> (buf: &mut dyn ReadableBuf, field_name: &str, validator: F) -> PacketParseResult<Bit> {
    let value = read_bit_field(buf, field_name)?;
    match validator(&value) {
        Ok(_) => Ok(value),
        Err(e) => Err(PacketParseError::ValidationError {
            field_name: field_name.to_owned(),
            msg: e.0
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_bit_field() {
        //let data: Vec<u8> = vec![0xDE, 0xAD];
        let data = Vec::<u8>::new();

        let mut buf = BitBuffer::new(data);

        if let Err(e) = read_bit_field(&mut buf, "my field") {
            println!("Error parsing: {}", e);
        }
    }

    #[test]
    fn test_validate() {
        let validate_bit: fn(&Bit) -> ValidationResult = |bit| {
            match bit {
                Bit::One => Ok(()),
                Bit::Zero => Err(ValidationError("Expected value 1, found value 0".to_string()))
            }
        };
        let data: Vec<u8> = vec![0b0];
        let mut buf = BitBuffer::new(data);

        if let Err(e) = read_and_validate_bit_field(&mut buf, "my bit", validate_bit) {
            println!("Error parsing: {}", e);
        }
        
    }
}
