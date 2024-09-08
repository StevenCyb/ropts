use std::fmt;

/// An error type for validation and parsing errors.
///
/// # Examples
///
/// ```
/// use ropts::error::Error;
///
/// let validation_error = Error::Validation("Invalid input".to_string());
/// assert_eq!(format!("{}", validation_error), "Validation error: Invalid input");
///
/// let parsing_error = Error::Parsing("Failed to parse".to_string());
/// assert_eq!(format!("{}", parsing_error), "Parsing error: Failed to parse");
/// ```
#[derive(Debug, Clone)]
pub enum Error {
    Validation(String),
    Parsing(String),
}

impl fmt::Display for Error {
    /// Formats the error for display.
    ///
    /// # Examples
    ///
    /// ```
    /// use ropts::error::Error;
    ///
    /// let error = Error::Validation("Invalid input".to_string());
    /// assert_eq!(format!("{}", error), "Validation error: Invalid input");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Validation(msg) => write!(f, "Validation error: {}", msg),
            Error::Parsing(msg) => write!(f, "Parsing error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_validation_display() {
        let err = Error::Validation("test".to_string());
        assert_eq!(format!("{}", err), "Validation error: test");
    }

    #[test]
    fn error_parsing_display() {
        let err = Error::Parsing("test".to_string());
        assert_eq!(format!("{}", err), "Parsing error: test");
    }
}
