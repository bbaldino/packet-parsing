use bitbuffer::{bit::Bit, readable_buf::ReadableBuf};

use crate::{
    error::{PacketParseResult, ValidationResult},
    packet_parsing::{
        read_bit_field, read_bit_field_and_validate, read_bits_as_u8_field,
        read_bits_as_u8_field_and_validate, read_bool_field, read_bool_field_and_validate,
        read_bytes_field, read_bytes_field_and_validate, read_u16_field,
        read_u16_field_and_validate, read_u24_field, read_u24_field_and_validate, read_u32_field,
        read_u32_field_and_validate, read_u8_field, read_u8_field_and_validate,
    },
};

pub trait FieldBuffer {
    fn read_bit_field(&mut self, field_name: &str) -> PacketParseResult<Bit>;
    fn read_bit_field_and_validate<F: FnOnce(&Bit) -> ValidationResult>(
        &mut self,
        field_name: &str,
        validator: F,
    ) -> PacketParseResult<Bit>;

    fn read_bool_field(&mut self, field_name: &str) -> PacketParseResult<bool>;
    fn read_bool_field_and_validate<F: FnOnce(bool) -> ValidationResult>(
        &mut self,
        field_name: &str,
        validator: F,
    ) -> PacketParseResult<bool>;

    fn read_bits_as_u8_field(&mut self, num_bits: usize, field_name: &str)
        -> PacketParseResult<u8>;
    fn read_bits_as_u8_field_and_validate<F: FnOnce(u8) -> ValidationResult>(
        &mut self,
        num_bits: usize,
        field_name: &str,
        validator: F,
    ) -> PacketParseResult<u8>;

    fn read_u8_field(&mut self, field_name: &str) -> PacketParseResult<u8>;
    fn read_u8_field_and_validate<F: FnOnce(u8) -> ValidationResult>(
        &mut self,
        field_name: &str,
        validator: F,
    ) -> PacketParseResult<u8>;

    fn read_u16_field(&mut self, field_name: &str) -> PacketParseResult<u16>;
    fn read_u16_field_and_validate<F: FnOnce(u16) -> ValidationResult>(
        &mut self,
        field_name: &str,
        validator: F,
    ) -> PacketParseResult<u16>;

    fn read_u24_field(&mut self, field_name: &str) -> PacketParseResult<u32>;
    fn read_u24_field_and_validate<F: FnOnce(u32) -> ValidationResult>(
        &mut self,
        field_name: &str,
        validator: F,
    ) -> PacketParseResult<u32>;

    fn read_u32_field(&mut self, field_name: &str) -> PacketParseResult<u32>;
    fn read_u32_field_and_validate<F: FnOnce(u32) -> ValidationResult>(
        &mut self,
        field_name: &str,
        validator: F,
    ) -> PacketParseResult<u32>;

    fn read_bytes_field(&mut self, num_bytes: usize, field_name: &str) -> PacketParseResult<&[u8]>;
    fn read_bytes_field_and_validate<F: FnOnce(&[u8]) -> ValidationResult>(
        &mut self,
        num_bytes: usize,
        field_name: &str,
        validator: F,
    ) -> PacketParseResult<&[u8]>;
}

/// This makes the field methods more convenient to use with a ReadableBuf
impl<'a> FieldBuffer for dyn ReadableBuf + 'a {
    fn read_bit_field(&mut self, field_name: &str) -> PacketParseResult<Bit> {
        read_bit_field(self, field_name)
    }

    fn read_bit_field_and_validate<F: FnOnce(&Bit) -> ValidationResult>(
        &mut self,
        field_name: &str,
        validator: F,
    ) -> PacketParseResult<Bit> {
        read_bit_field_and_validate(self, field_name, validator)
    }

    fn read_bool_field(&mut self, field_name: &str) -> PacketParseResult<bool> {
        read_bool_field(self, field_name)
    }

    fn read_bool_field_and_validate<F: FnOnce(bool) -> ValidationResult>(
        &mut self,
        field_name: &str,
        validator: F,
    ) -> PacketParseResult<bool> {
        read_bool_field_and_validate(self, field_name, validator)
    }

    fn read_bits_as_u8_field(
        &mut self,
        num_bits: usize,
        field_name: &str,
    ) -> PacketParseResult<u8> {
        read_bits_as_u8_field(self, num_bits, field_name)
    }

    fn read_bits_as_u8_field_and_validate<F: FnOnce(u8) -> ValidationResult>(
        &mut self,
        num_bits: usize,
        field_name: &str,
        validator: F,
    ) -> PacketParseResult<u8> {
        read_bits_as_u8_field_and_validate(self, field_name, num_bits, validator)
    }

    fn read_u8_field(&mut self, field_name: &str) -> PacketParseResult<u8> {
        read_u8_field(self, field_name)
    }

    fn read_u8_field_and_validate<F: FnOnce(u8) -> ValidationResult>(
        &mut self,
        field_name: &str,
        validator: F,
    ) -> PacketParseResult<u8> {
        read_u8_field_and_validate(self, field_name, validator)
    }

    fn read_u16_field(&mut self, field_name: &str) -> PacketParseResult<u16> {
        read_u16_field(self, field_name)
    }

    fn read_u16_field_and_validate<F: FnOnce(u16) -> ValidationResult>(
        &mut self,
        field_name: &str,
        validator: F,
    ) -> PacketParseResult<u16> {
        read_u16_field_and_validate(self, field_name, validator)
    }

    fn read_u24_field(&mut self, field_name: &str) -> PacketParseResult<u32> {
        read_u24_field(self, field_name)
    }

    fn read_u24_field_and_validate<F: FnOnce(u32) -> ValidationResult>(
        &mut self,
        field_name: &str,
        validator: F,
    ) -> PacketParseResult<u32> {
        read_u24_field_and_validate(self, field_name, validator)
    }

    fn read_u32_field(&mut self, field_name: &str) -> PacketParseResult<u32> {
        read_u32_field(self, field_name)
    }

    fn read_u32_field_and_validate<F: FnOnce(u32) -> ValidationResult>(
        &mut self,
        field_name: &str,
        validator: F,
    ) -> PacketParseResult<u32> {
        read_u32_field_and_validate(self, field_name, validator)
    }

    fn read_bytes_field(&mut self, num_bytes: usize, field_name: &str) -> PacketParseResult<&[u8]> {
        read_bytes_field(self, num_bytes, field_name)
    }

    fn read_bytes_field_and_validate<F: FnOnce(&[u8]) -> ValidationResult>(
        &mut self,
        num_bytes: usize,
        field_name: &str,
        validator: F,
    ) -> PacketParseResult<&[u8]> {
        read_bytes_field_and_validate(self, num_bytes, field_name, validator)
    }
}
