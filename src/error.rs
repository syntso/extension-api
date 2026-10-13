use crate::bindings::synt::extension::types;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{code}: {message}")]
pub struct Error {
    code: String,
    message: String,
}

impl Error {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<types::Error> for Error {
    fn from(error: types::Error) -> Self {
        Self::new(error.code, error.message)
    }
}

impl From<Error> for types::Error {
    fn from(error: Error) -> Self {
        Self {
            code: error.code,
            message: error.message,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shows_code_and_message() {
        let error = Error::new("not_found", "no such host");

        assert_eq!(error.to_string(), "not_found: no such host");
    }

    #[test]
    fn round_trips_through_the_interface() {
        let error = Error::new("invalid_params", "port is missing");

        let wire = types::Error::from(error.clone());
        let back = Error::from(wire);

        assert_eq!(back, error);
    }
}
