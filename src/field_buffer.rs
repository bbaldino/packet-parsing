use bitbuffer::{bit::Bit, readable_buf::ReadableBuf};

use crate::{
    error::PacketParseResult,
    packet_parsing::{read_bit_field, read_bit_field_as_bool},
};

pub trait FieldBuffer {
    fn read_bit_field(&mut self, field_name: &str) -> PacketParseResult<Bit>;
    fn read_bool_field(&mut self, field_name: &str) -> PacketParseResult<bool>;
}

/// This makes the field methods more convenient to use with a ReadableBuf
impl<'a> FieldBuffer for dyn ReadableBuf + 'a {
    fn read_bit_field(&mut self, field_name: &str) -> PacketParseResult<Bit> {
        read_bit_field(self, field_name)
    }

    fn read_bool_field(&mut self, field_name: &str) -> PacketParseResult<bool> {
        read_bit_field_as_bool(self, field_name)
    }
}
