use bitbuffer::error::{BitBufferError, BitBufferResult};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PacketParseError {
    #[error("Unable to parse field '{field_name}': {source}")]
    BufferError {
        field_name: String,
        source: BitBufferError,
    },
    #[error("Validation failed for field '{field_name}': '{msg}'")]
    ValidationError { field_name: String, msg: String },
    #[error("Erorr parsing field group '{field_group_name}': {source}")]
    FieldGroupParsingError {
        field_group_name: String,
        source: Box<PacketParseError>,
    },
}

pub struct ValidationError(pub String);

pub type PacketParseResult<T> = Result<T, PacketParseError>;
pub type ValidationResult = Result<(), ValidationError>;

pub trait ToPacketParseError {
    fn into_ppe(self, field_name: &str) -> PacketParseError;
}

impl ToPacketParseError for BitBufferError {
    fn into_ppe(self, field_name: &str) -> PacketParseError {
        PacketParseError::BufferError {
            field_name: field_name.to_owned(),
            source: self,
        }
    }
}

impl ToPacketParseError for ValidationError {
    fn into_ppe(self, field_name: &str) -> PacketParseError {
        PacketParseError::ValidationError {
            field_name: field_name.to_owned(),
            msg: self.0,
        }
    }
}

pub trait ToPacketParseResult<T> {
    fn to_ppr(self, field_name: &str) -> PacketParseResult<T>;
}

impl<T> ToPacketParseResult<T> for BitBufferResult<T> {
    fn to_ppr(self, field_name: &str) -> PacketParseResult<T> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(e.into_ppe(field_name)),
        }
    }
}
