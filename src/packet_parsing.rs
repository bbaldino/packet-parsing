use crate::error::{
    PacketParseError, PacketParseResult, ToPacketParseError, ToPacketParseResult, ValidationResult,
};
use bitbuffer::bit::Bit;
use bitbuffer::readable_buf::ReadableBuf;

pub fn read_bit_field(buf: &mut dyn ReadableBuf, field_name: &str) -> PacketParseResult<Bit> {
    buf.read_bit().to_ppr(field_name)
}

pub fn read_bit_field_and_validate<F: FnOnce(&Bit) -> ValidationResult>(
    buf: &mut dyn ReadableBuf,
    field_name: &str,
    validator: F,
) -> PacketParseResult<Bit> {
    let value = read_bit_field(buf, field_name)?;
    match validator(&value) {
        Ok(_) => Ok(value),
        Err(e) => Err(e.into_ppe(field_name)),
    }
}

pub fn read_bool_field(buf: &mut dyn ReadableBuf, field_name: &str) -> PacketParseResult<bool> {
    buf.read_bit_as_bool().to_ppr(field_name)
}

pub fn read_bool_field_and_validate<F: FnOnce(bool) -> ValidationResult>(
    buf: &mut dyn ReadableBuf,
    field_name: &str,
    validator: F,
) -> PacketParseResult<bool> {
    let value = read_bool_field(buf, field_name)?;
    match validator(value) {
        Ok(_) => Ok(value),
        Err(e) => Err(e.into_ppe(field_name)),
    }
}

pub fn read_bits_as_u8_field(
    buf: &mut dyn ReadableBuf,
    num_bits: usize,
    field_name: &str,
) -> PacketParseResult<u8> {
    buf.read_bits_as_u8(num_bits).to_ppr(field_name)
}

pub fn read_bits_as_u8_field_and_validate<F: FnOnce(u8) -> ValidationResult>(
    buf: &mut dyn ReadableBuf,
    field_name: &str,
    num_bits: usize,
    validator: F,
) -> PacketParseResult<u8> {
    let value = read_bits_as_u8_field(buf, num_bits, field_name)?;
    match validator(value) {
        Ok(_) => Ok(value),
        Err(e) => Err(e.into_ppe(field_name)),
    }
}

pub fn read_u8_field(buf: &mut dyn ReadableBuf, field_name: &str) -> PacketParseResult<u8> {
    buf.read_u8().to_ppr(field_name)
}

pub fn read_u8_field_and_validate<F: FnOnce(u8) -> ValidationResult>(
    buf: &mut dyn ReadableBuf,
    field_name: &str,
    validator: F,
) -> PacketParseResult<u8> {
    let value = read_u8_field(buf, field_name)?;
    match validator(value) {
        Ok(_) => Ok(value),
        Err(e) => Err(e.into_ppe(field_name)),
    }
}

pub fn read_u16_field(buf: &mut dyn ReadableBuf, field_name: &str) -> PacketParseResult<u16> {
    buf.read_u16().to_ppr(field_name)
}

pub fn read_u16_field_and_validate<F: FnOnce(u16) -> ValidationResult>(
    buf: &mut dyn ReadableBuf,
    field_name: &str,
    validator: F,
) -> PacketParseResult<u16> {
    let value = read_u16_field(buf, field_name)?;
    match validator(value) {
        Ok(_) => Ok(value),
        Err(e) => Err(e.into_ppe(field_name)),
    }
}

pub fn try_parse_field_group<T, F: FnOnce() -> PacketParseResult<T>>(
    field_group_name: &str,
    block: F,
) -> PacketParseResult<T> {
    match block() {
        Ok(r) => Ok(r),
        Err(e) => Err(PacketParseError::FieldGroupParsingError {
            field_group_name: field_group_name.to_owned(),
            source: Box::new(e),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ValidationError;
    use bitbuffer::bit_buffer::BitBuffer;

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
        let validate_bit: fn(&Bit) -> ValidationResult = |bit| match bit {
            Bit::One => Ok(()),
            Bit::Zero => Err(ValidationError(
                "Expected value 1, found value 0".to_string(),
            )),
        };
        let data: Vec<u8> = vec![0b0];
        let mut buf = BitBuffer::new(data);

        if let Err(e) = read_bit_field_and_validate(&mut buf, "my bit", validate_bit) {
            println!("Error parsing: {}", e);
        }
    }

    #[test]
    fn test_parse_field_group() {
        println!("Parsing field group");
        fn validate_bit(bit: &Bit) -> ValidationResult {
            println!("Validating bit: {:?}", bit);
            match bit {
                Bit::One => Ok(()),
                Bit::Zero => {
                    println!("Returning validation error");
                    Err(ValidationError(
                        "Expected value 1, found value 0".to_string(),
                    ))
                }
            }
        }

        let data: Vec<u8> = vec![0b10000000];
        let mut buf = BitBuffer::new(data);

        #[derive(Debug)]
        struct Header {
            version: Bit,
            padding: Bit,
        }

        fn parse_header(buf: &mut dyn ReadableBuf) -> PacketParseResult<Header> {
            Ok(Header {
                version: read_bit_field_and_validate(buf, "version", validate_bit)?,
                padding: read_bit_field_and_validate(buf, "padding", validate_bit)?,
            })
        }

        if let Err(e) = try_parse_field_group("header", || Ok(parse_header(&mut buf)?)) {
            println!("Error parsing: {}", e);
        }
    }
}
