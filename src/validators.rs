use std::error::Error;
use std::fmt::{Debug, Display};
use std::ops::RangeBounds;

use crate::error::{RequireEqualError, ValidationError, ValidationResult};

/// Expose a 'validate' function on any value |T|, such that a block can be easily chained from the
/// value to provide a Result describing whether or not the validation succeeded.
pub trait Validatable<T> {
    fn validate<F: FnOnce(&T) -> ValidationResult>(self, validator: F)
        -> Result<T, Box<dyn Error>>;
}

impl<T> Validatable<T> for T {
    fn validate<F: FnOnce(&T) -> ValidationResult>(
        self,
        validator: F,
    ) -> Result<T, Box<dyn Error>> {
        match validator(&self) {
            Ok(_) => Ok(self),
            Err(e) => Err(e.into()),
        }
    }
}

/// Expose a 'require_equal' validator helper as a shortcut to validate that a parsed value equals
/// what is expected.
pub trait RequireEqual<T> {
    fn require_value(self, expected: T) -> Result<T, Box<dyn Error>>;
}

impl<T> RequireEqual<T> for T
where
    T: Eq + Display,
{
    fn require_value(self, expected: T) -> Result<T, Box<dyn Error>> {
        if self == expected {
            Ok(self)
        } else {
            Err(RequireEqualError(format!(
                "A value of {} was required, got value {}",
                expected, self
            ))
            .into())
        }
    }
}

pub trait RequireInRange<T, R: RangeBounds<T>> {
    fn require_in_range(self, r: R) -> Result<T, Box<dyn Error>>;
}

impl<T, R> RequireInRange<T, R> for T
where
    T: Display + PartialOrd + Debug,
    R: RangeBounds<T>,
{
    fn require_in_range(self, r: R) -> Result<T, Box<dyn Error>> {
        match r.contains(&self) {
            true => Ok(self),
            false => Err(ValidationError(format!(
                "Expected value {} to be within range {:?}..{:?}",
                self,
                r.start_bound(),
                r.end_bound()
            ))
            .into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_value() {
        assert!(true.require_value(true).is_ok());
        assert!(2.require_value(3).is_err());
    }

    #[test]
    fn test_require_in_range() {
        let value = 42u8;

        assert_eq!(value.require_in_range(0..=42).unwrap(), 42);
        assert!(value.require_in_range(0..42).is_err());
    }
}
