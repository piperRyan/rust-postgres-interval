use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    ParseIntErr(ParseIntError),
    ParseFloatErr(ParseFloatError),
    InvalidYearMonth(String),
    InvalidTime(String),
    InvalidInterval(String),
}

impl<'a> ParseError {
    pub fn from_year_month(message: &'a str) -> ParseError {
        ParseError::InvalidYearMonth(String::from(message))
    }

    pub fn from_time(message: &'a str) -> ParseError {
        ParseError::InvalidTime(String::from(message))
    }

    pub fn from_invalid_interval(message: &'a str) -> ParseError {
        ParseError::InvalidInterval(String::from(message))
    }
}

impl From<ParseIntError> for ParseError {
    fn from(error: ParseIntError) -> ParseError {
        ParseError::ParseIntErr(error)
    }
}

impl From<ParseFloatError> for ParseError {
    fn from(error: ParseFloatError) -> ParseError {
        ParseError::ParseFloatErr(error)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::ParseIntErr(e) => write!(f, "Failed to parse integer: {}", e),
            ParseError::ParseFloatErr(e) => write!(f, "Failed to parse float: {}", e),
            ParseError::InvalidYearMonth(s) => write!(f, "Invalid year/month interval: {}", s),
            ParseError::InvalidTime(s) => write!(f, "Invalid time interval: {}", s),
            ParseError::InvalidInterval(s) => write!(f, "Invalid interval: {}", s),
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseError::ParseIntErr(e) => Some(e),
            ParseError::ParseFloatErr(e) => Some(e),
            ParseError::InvalidYearMonth(_)
            | ParseError::InvalidTime(_)
            | ParseError::InvalidInterval(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ParseError;
    #[test]
    fn can_covert_from_parse_float_error() {
        let float_err = "fake".parse::<f64>().unwrap_err();
        let result = ParseError::from(float_err.clone());
        let expected = ParseError::ParseFloatErr(float_err);
        assert_eq!(result, expected);
    }

    #[test]
    fn can_covert_from_parse_int_error() {
        let float_err = "fake".parse::<i32>().unwrap_err();
        let result = ParseError::from(float_err.clone());
        let expected = ParseError::ParseIntErr(float_err);
        assert_eq!(result, expected);
    }
}
