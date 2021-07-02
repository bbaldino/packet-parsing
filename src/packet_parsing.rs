use std::fmt::Formatter;

use crate::error::{OperationResult, PacketParseError, PacketParseResult, ValidationResult};
use bitbuffer::bit::Bit;
use bitbuffer::readable_buf::ReadableBuf;

fn try_parse_field<T, U, F: FnOnce() -> Result<T, U>>(
    field_name: &str,
    block: F,
) -> PacketParseResult<T>
where
    U: Into<Box<dyn std::error::Error>>,
{
    match block() {
        Ok(v) => Ok(v),
        Err(e) => Err(PacketParseError::FieldParseError {
            field_name: field_name.to_owned(),
            error: e.into(),
        }),
    }
}

trait Validatable<T> {
    fn validate<F: FnOnce(T) -> ValidationResult>(self, validator: F) -> OperationResult<T>;
}

impl<T> Validatable<T> for T
where
    T: Copy,
{
    fn validate<F: FnOnce(T) -> ValidationResult>(self, validator: F) -> OperationResult<T> {
        match validator(self) {
            Ok(_) => Ok(self),
            Err(e) => Err(e.into()),
        }
    }
}

//impl<T, E> Validatable<T> for Result<T, E>
//where
//    E: Into<Box<dyn std::error::Error>>,
//{
//    fn validate<F: FnOnce(T) -> ValidationResult>(self, validator: F) -> OperationResult<T> {
//        match self {
//            Ok(v) => match validator(v) {
//                Ok(_) => Ok(v),
//                Err(e) => Err(Box::new(e)),
//            },
//            Err(e) => Err(e.into()),
//        }
//    }
//}

//pub fn read_bit_field(buf: &mut dyn ReadableBuf, field_name: &str) -> PacketParseResult<Bit> {
//    buf.read_bit().to_ppr(field_name)
//}
//
//pub fn read_bit_field_and_validate<F: FnOnce(&Bit) -> ValidationResult>(
//    buf: &mut dyn ReadableBuf,
//    field_name: &str,
//    validator: F,
//) -> PacketParseResult<Bit> {
//    let value = read_bit_field(buf, field_name)?;
//    match validator(&value) {
//        Ok(_) => Ok(value),
//        Err(e) => Err(e.into_ppe(field_name)),
//    }
//}
//
//pub fn read_bool_field(buf: &mut dyn ReadableBuf, field_name: &str) -> PacketParseResult<bool> {
//    buf.read_bit_as_bool().to_ppr(field_name)
//}
//
//pub fn read_bool_field_and_validate<F: FnOnce(bool) -> ValidationResult>(
//    buf: &mut dyn ReadableBuf,
//    field_name: &str,
//    validator: F,
//) -> PacketParseResult<bool> {
//    let value = read_bool_field(buf, field_name)?;
//    match validator(value) {
//        Ok(_) => Ok(value),
//        Err(e) => Err(e.into_ppe(field_name)),
//    }
//}
//
//pub fn read_bits_as_u8_field(
//    buf: &mut dyn ReadableBuf,
//    num_bits: usize,
//    field_name: &str,
//) -> PacketParseResult<u8> {
//    buf.read_bits_as_u8(num_bits).to_ppr(field_name)
//}
//
//pub fn read_bits_as_u8_field_and_validate<F: FnOnce(u8) -> ValidationResult>(
//    buf: &mut dyn ReadableBuf,
//    field_name: &str,
//    num_bits: usize,
//    validator: F,
//) -> PacketParseResult<u8> {
//    let value = read_bits_as_u8_field(buf, num_bits, field_name)?;
//    match validator(value) {
//        Ok(_) => Ok(value),
//        Err(e) => Err(e.into_ppe(field_name)),
//    }
//}
//
//pub fn read_u8_field(buf: &mut dyn ReadableBuf, field_name: &str) -> PacketParseResult<u8> {
//    buf.read_u8().to_ppr(field_name)
//}
//
//pub fn read_u8_field_and_validate<F: FnOnce(u8) -> ValidationResult>(
//    buf: &mut dyn ReadableBuf,
//    field_name: &str,
//    validator: F,
//) -> PacketParseResult<u8> {
//    let value = read_u8_field(buf, field_name)?;
//    match validator(value) {
//        Ok(_) => Ok(value),
//        Err(e) => Err(e.into_ppe(field_name)),
//    }
//}
//
//pub fn read_u16_field(buf: &mut dyn ReadableBuf, field_name: &str) -> PacketParseResult<u16> {
//    buf.read_u16().to_ppr(field_name)
//}
//
//pub fn read_u16_field_and_validate<F: FnOnce(u16) -> ValidationResult>(
//    buf: &mut dyn ReadableBuf,
//    field_name: &str,
//    validator: F,
//) -> PacketParseResult<u16> {
//    let value = read_u16_field(buf, field_name)?;
//    match validator(value) {
//        Ok(_) => Ok(value),
//        Err(e) => Err(e.into_ppe(field_name)),
//    }
//}
//
//pub fn read_u24_field(buf: &mut dyn ReadableBuf, field_name: &str) -> PacketParseResult<u32> {
//    buf.read_u24().to_ppr(field_name)
//}
//
//pub fn read_u24_field_and_validate<F: FnOnce(u32) -> ValidationResult>(
//    buf: &mut dyn ReadableBuf,
//    field_name: &str,
//    validator: F,
//) -> PacketParseResult<u32> {
//    let value = read_u24_field(buf, field_name)?;
//    match validator(value) {
//        Ok(_) => Ok(value),
//        Err(e) => Err(e.into_ppe(field_name)),
//    }
//}
//
//pub fn read_u32_field(buf: &mut dyn ReadableBuf, field_name: &str) -> PacketParseResult<u32> {
//    buf.read_u32().to_ppr(field_name)
//}
//
//pub fn read_u32_field_and_validate<F: FnOnce(u32) -> ValidationResult>(
//    buf: &mut dyn ReadableBuf,
//    field_name: &str,
//    validator: F,
//) -> PacketParseResult<u32> {
//    let value = read_u32_field(buf, field_name)?;
//    match validator(value) {
//        Ok(_) => Ok(value),
//        Err(e) => Err(e.into_ppe(field_name)),
//    }
//}
//
//pub fn read_bytes_field<'a, 'b>(
//    buf: &'a mut dyn ReadableBuf,
//    num_bytes: usize,
//    field_name: &str,
//) -> PacketParseResult<&'b [u8]>
//where
//    'a: 'b,
//{
//    buf.read_bytes(num_bytes).to_ppr(field_name)
//}
//
//pub fn read_bytes_field_and_validate<'a, 'b, F: FnOnce(&[u8]) -> ValidationResult>(
//    buf: &'a mut dyn ReadableBuf,
//    num_bytes: usize,
//    field_name: &str,
//    validator: F,
//) -> PacketParseResult<&'b [u8]>
//where
//    'a: 'b,
//{
//    let value = read_bytes_field(buf, num_bytes, field_name)?;
//    match validator(value) {
//        Ok(_) => Ok(value),
//        Err(e) => Err(e.into_ppe(field_name)),
//    }
//}
//
//pub fn try_parse_field_group<T, F: FnOnce() -> PacketParseResult<T>>(
//    field_group_name: &str,
//    block: F,
//) -> PacketParseResult<T> {
//    match block() {
//        Ok(r) => Ok(r),
//        Err(e) => Err(PacketParseError::FieldGroupParsingError {
//            field_group_name: field_group_name.to_owned(),
//            source: Box::new(e),
//        }),
//    }
//}

#[derive(Debug)]
struct Foo;

impl std::fmt::Display for Foo {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}

impl std::error::Error for Foo {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        todo!()
    }
}

// works fine
fn foo(buf: &mut dyn ReadableBuf) -> Result<u8, Box<dyn std::error::Error>> {
    let x = buf.read_u8()?;
    Ok(x)
}

fn bar(buf: &mut dyn ReadableBuf) -> OperationResult<u8> {
    Ok(buf.read_u8()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ValidationError;
    use bitbuffer::bit_buffer::BitBuffer;

    fn validate_field(value: u16) -> ValidationResult {
        println!("validating value {}", value);
        match value {
            0..=5 => Ok(()),
            v @ _ => Err(ValidationError(format!(
                "Expected value between 0 and 5, got {}",
                v
            ))),
        }
    }

    fn validate_version(value: u8) -> ValidationResult {
        match value {
            2 => Ok(()),
            v @ _ => Err(ValidationError(format!("Expected version=2, got {}", v))),
        }
    }

    fn validate_packet_type(value: u8) -> ValidationResult {
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

    fn parse_header(buf: &mut dyn ReadableBuf) -> PacketParseResult<Header> {
        Ok(Header {
            version: try_parse_field("version", || {
                buf.read_bits_as_u8(2)?.validate(validate_version)
            })?,
            has_padding: try_parse_field("has_padding", || buf.read_bit_as_bool())?,
            report_count: try_parse_field("report count", || buf.read_bits_as_u8(5))?,
            packet_type: try_parse_field("packet type", || {
                buf.read_u8()?.validate(validate_packet_type)
            })?,
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

    //#[test]
    //fn test_read_bit_field() {
    //    let mut buf = BitBuffer::new(vec![0b11000000]);

    //    let res = read_bit_field(&mut buf, "my field");
    //    assert!(res.is_ok());
    //    assert_eq!(Bit::One, res.unwrap());
    //}

    //#[test]
    //fn test_read_bit_failure() {
    //    let mut buf = BitBuffer::new(Vec::new());

    //    let res = read_bit_field(&mut buf, "my field");
    //    assert!(res.is_err());
    //    let r = res.unwrap_err();
    //    assert!(matches!(r, PacketParseError::BufferError { .. }));
    //}

    //#[test]
    //fn test_validate() {
    //    let validate_bit: fn(&Bit) -> ValidationResult = |bit| match bit {
    //        Bit::One => Ok(()),
    //        Bit::Zero => Err(ValidationError(
    //            "Expected value 1, found value 0".to_string(),
    //        )),
    //    };
    //    let data: Vec<u8> = vec![0b0];
    //    let mut buf = BitBuffer::new(data);

    //    if let Err(e) = read_bit_field_and_validate(&mut buf, "my bit", validate_bit) {
    //        println!("Error parsing: {}", e);
    //    }
    //}

    //#[test]
    //fn test_parse_field_group() {
    //    println!("Parsing field group");
    //    fn validate_bit(bit: &Bit) -> ValidationResult {
    //        println!("Validating bit: {:?}", bit);
    //        match bit {
    //            Bit::One => Ok(()),
    //            Bit::Zero => {
    //                println!("Returning validation error");
    //                Err(ValidationError(
    //                    "Expected value 1, found value 0".to_string(),
    //                ))
    //            }
    //        }
    //    }

    //    let data: Vec<u8> = vec![0b10000000];
    //    let mut buf = BitBuffer::new(data);

    //    #[derive(Debug)]
    //    struct Header {
    //        version: Bit,
    //        padding: Bit,
    //    }

    //    fn parse_header(buf: &mut dyn ReadableBuf) -> PacketParseResult<Header> {
    //        Ok(Header {
    //            version: read_bit_field_and_validate(buf, "version", validate_bit)?,
    //            padding: read_bit_field_and_validate(buf, "padding", validate_bit)?,
    //        })
    //    }

    //    if let Err(e) = try_parse_field_group("header", || Ok(parse_header(&mut buf)?)) {
    //        println!("Error parsing: {}", e);
    //    }
    //}
}
