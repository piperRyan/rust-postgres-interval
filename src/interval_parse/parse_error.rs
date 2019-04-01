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
