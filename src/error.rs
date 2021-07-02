use bitbuffer::error::{BitBufferError, BitBufferResult};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PacketParseError {
    #[error("Error parsing field '{field_name}': {error}")]
    FieldParseError {
        field_name: String,
        error: Box<dyn std::error::Error>,
    },
    #[error("Erorr parsing field group '{field_group_name}': {error}")]
    FieldGroupParsingError {
        field_group_name: String,
        error: Box<PacketParseError>,
    },
}

#[derive(Error, Debug)]
#[error("Validation error: {0}")]
pub struct ValidationError(pub String);

/// The result from any sort of operation that might be done (parsing, validating, etc.)
/// This type does not carry any context of what was being attempted at the time of
/// failure
pub type OperationResult<T> = Result<T, Box<dyn std::error::Error>>;

pub type PacketParseResult<T> = Result<T, PacketParseError>;
pub type ValidationResult = Result<(), ValidationError>;

//pub trait ToPacketParseError {
//    fn into_ppe(self, field_name: &str) -> PacketParseError;
//}

//impl ToPacketParseError for BitBufferError {
//    fn into_ppe(self, field_name: &str) -> PacketParseError {
//        PacketParseError::BufferError {
//            field_name: field_name.to_owned(),
//            source: self,
//        }
//    }
//}
//
//impl ToPacketParseError for ValidationError {
//    fn into_ppe(self, field_name: &str) -> PacketParseError {
//        PacketParseError::ValidationError {
//            field_name: field_name.to_owned(),
//            msg: self.0,
//        }
//    }
//}

pub trait ToPacketParseResult<T> {
    fn to_ppr(self, field_name: &str) -> PacketParseResult<T>;
}

//impl<T> ToPacketParseResult<T> for BitBufferResult<T> {
//    fn to_ppr(self, field_name: &str) -> PacketParseResult<T> {
//        match self {
//            Ok(t) => Ok(t),
//            Err(e) => Err(e.into_ppe(field_name)),
//        }
//    }
//}
