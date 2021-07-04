use thiserror::Error;

#[derive(Error, Debug)]
#[error("Error parsing field '{field_name}': {error}")]
pub struct PacketParseError {
    pub field_name: String,
    pub error: Box<dyn std::error::Error>,
}

#[derive(Error, Debug)]
#[error("Validation error: {0}")]
pub struct ValidationError(pub String);

#[derive(Error, Debug)]
#[error("{0}")]
pub struct RequireEqualError(pub String);

pub type PacketParseResult<T> = Result<T, Box<dyn std::error::Error>>;
pub type ValidationResult = Result<(), ValidationError>;
