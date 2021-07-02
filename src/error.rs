use thiserror::Error;

#[derive(Error, Debug)]
pub enum PacketParseError {
    #[error("Error parsing field '{field_name}': {error}")]
    FieldParseError {
        field_name: String,
        error: Box<dyn std::error::Error>,
    },
}

#[derive(Error, Debug)]
#[error("Validation error: {0}")]
pub struct ValidationError(pub String);

pub type PacketParseResult<T> = Result<T, PacketParseError>;
pub type ValidationResult<T> = Result<T, ValidationError>;

pub fn wrap<T: std::error::Error + 'static>(
    field_name: &str,
    err: T,
) -> Box<dyn std::error::Error> {
    PacketParseError::FieldParseError {
        field_name: field_name.to_owned(),
        error: err.into(),
    }
    .into()
}

// TODO: maybe I can enforce the trait bounds Result by creating a struct, enforcing the boundaries
// on that struct, and then implementing From<Struct> for Result<T, E>.  Then I would return the
// struct type? Will that be too weird compared to returning a Result?
