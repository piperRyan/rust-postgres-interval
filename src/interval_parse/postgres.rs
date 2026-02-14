use super::parse_error::ParseError;
use crate::{Interval, interval_norm::IntervalNorm};

use super::{
    DAYS_PER_MONTH, HOURS_PER_DAY, MICROS_PER_SECOND, MINUTES_PER_HOUR, MONTHS_PER_YEAR,
    SECONDS_PER_MIN, scale_date, scale_time,
};

impl Interval {
    pub fn from_postgres(iso_str: &str) -> Result<Interval, ParseError> {
        let mut delim = vec![
            "years", "months", "mons", "days", "hours", "minutes", "seconds",
        ];
        let mut time_tokens = iso_str.split(' ').collect::<Vec<&str>>(); // clean up empty values caused by n spaces between values.
        time_tokens.retain(|&token| !token.is_empty());
        // since there might not be space between the delim and the
        // value we need to scan each token.
        let mut final_tokens = Vec::with_capacity(time_tokens.len());
        for token in time_tokens {
            if is_token_alphanumeric(token)? {
                let (val, unit) = split_token(token)?;
                final_tokens.push(val);
                final_tokens.push(unit);
            } else {
                final_tokens.push(token.to_owned());
            }
        }
        if final_tokens.len() % 2 != 0 {
            return Err(ParseError::from_invalid_interval(
                "Invalid amount tokens were found.",
            ));
        }
        // Consume our tokens and build up the
        // normalized interval.
        let mut val = 0.0;
        let mut is_numeric = true;
        let mut interval = IntervalNorm::default();
        for token in final_tokens {
            if is_numeric {
                val = token.parse::<f64>()?;
                is_numeric = false;
            } else {
                consume_token(&mut interval, val, token, &mut delim)?;
                is_numeric = true;
            }
        }
        interval.try_into_interval()
    }
}

/// Does the token contain both alphabetic and numeric characters?
fn is_token_alphanumeric(val: &str) -> Result<bool, ParseError> {
    let mut has_numeric = false;
    let mut has_alpha = false;
    for character in val.chars() {
        if character.is_numeric() || character == '-' || character == '.' {
            has_numeric = true;
        } else if character.is_alphabetic() {
            has_alpha = true;
        } else {
            return Err(ParseError::from_invalid_interval(
                "String can only contain alpha numeric characters.",
            ));
        }
    }
    Ok(has_numeric && has_alpha)
}

/// Split the token into two tokens as they might of not been
/// seperated by a space.
fn split_token(val: &str) -> Result<(String, String), ParseError> {
    let mut is_numeric_done = false;
    let mut value = String::new();
    let mut delim = String::new();
    for character in val.chars() {
        if (character.is_numeric() || character == '-' || character == '.') && !is_numeric_done {
            value.push(character);
        } else if character.is_alphabetic() {
            is_numeric_done = true;
            delim.push(character)
        } else {
            return Err(ParseError::from_invalid_interval(
                "String can only contain alpha numeric characters.",
            ));
        }
    }
    Ok((value, delim))
}

/// Consume the token parts and add to the normalized interval.
fn consume_token(
    interval: &mut IntervalNorm,
    val: f64,
    delim: String,
    delim_list: &mut Vec<&str>,
) -> Result<(), ParseError> {
    // Unlike iso8601 the delimiter can only appear once
    // so we need to check if the token can be found in
    // the deliminator list.
    if delim_list.contains(&&*delim) {
        match &*delim {
            "years" => {
                let (year, month) = scale_date(val, MONTHS_PER_YEAR);
                interval.years += year;
                interval.months += month;
                delim_list.retain(|x| *x != "years");
                Ok(())
            }
            "months" | "mons" => {
                let (month, day) = scale_date(val, DAYS_PER_MONTH);
                interval.months += month;
                interval.days += day;
                delim_list.retain(|x| *x != "months" && *x != "mons");
                Ok(())
            }
            "days" => {
                let (days, hours) = scale_date(val, HOURS_PER_DAY);
                interval.days += days;
                interval.hours += hours as i64;
                delim_list.retain(|x| *x != "days");
                Ok(())
            }
            "hours" => {
                let (hours, minutes) = scale_time(val, MINUTES_PER_HOUR);
                interval.hours += hours;
                interval.minutes += minutes;
                delim_list.retain(|x| *x != "hours");
                Ok(())
            }
            "minutes" => {
                let (minutes, seconds) = scale_time(val, SECONDS_PER_MIN);
                interval.minutes += minutes;
                interval.seconds += seconds;
                delim_list.retain(|x| *x != "minutes");
                Ok(())
            }
            "seconds" => {
                let (seconds, microseconds) = scale_time(val, MICROS_PER_SECOND);
                interval.seconds += seconds;
                interval.microseconds += microseconds;
                delim_list.retain(|x| *x != "seconds");
                Ok(())
            }
            _ => unreachable!(),
        }
    } else {
        Err(ParseError::from_invalid_interval(&format!(
            "Unknown or duplicate deliminator \"{}\"",
            delim
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_postgres_1() {
        let interval = Interval::from_postgres("1 years").unwrap();
        let interval_exp = Interval::new(12, 0, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_2() {
        let interval = Interval::from_postgres("1years").unwrap();
        let interval_exp = Interval::new(12, 0, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_3() {
        let interval = Interval::from_postgres("1 years 1 months").unwrap();
        let interval_exp = Interval::new(13, 0, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_4() {
        let interval = Interval::from_postgres("1years 1months").unwrap();
        let interval_exp = Interval::new(13, 0, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_5() {
        let interval = Interval::from_postgres("1 years 1 mons 1 days").unwrap();
        let interval_exp = Interval::new(13, 1, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_6() {
        let interval = Interval::from_postgres("1years 1mons 1days").unwrap();
        let interval_exp = Interval::new(13, 1, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_7() {
        let interval = Interval::from_postgres("1 years 1 months 1 days 1 hours").unwrap();
        let interval_exp = Interval::new(13, 1, 3600000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_8() {
        let interval = Interval::from_postgres("1years 1months 1days 1hours").unwrap();
        let interval_exp = Interval::new(13, 1, 3600000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_9() {
        let interval =
            Interval::from_postgres("1 years 1 months 1 days 1 hours 10 minutes").unwrap();
        let interval_exp = Interval::new(13, 1, 4200000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_10() {
        let interval =
            Interval::from_postgres("1 years 1 months 1 days 1 hours 10 minutes 15 seconds")
                .unwrap();
        let interval_exp = Interval::new(13, 1, 4215000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_11() {
        let interval = Interval::from_postgres("1 hours").unwrap();
        let interval_exp = Interval::new(0, 0, 3600000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_12() {
        let interval = Interval::from_postgres("1 hours 10 minutes").unwrap();
        let interval_exp = Interval::new(0, 0, 4200000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_13() {
        let interval = Interval::from_postgres("1 hours 10 minutes 15 seconds").unwrap();
        let interval_exp = Interval::new(0, 0, 4215000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_14() {
        let interval = Interval::from_postgres("-1 years").unwrap();
        let interval_exp = Interval::new(-12, 0, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_15() {
        let interval = Interval::from_postgres("-1years").unwrap();
        let interval_exp = Interval::new(-12, 0, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_16() {
        let interval = Interval::from_postgres("-1 years -1 months").unwrap();
        let interval_exp = Interval::new(-13, 0, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_17() {
        let interval = Interval::from_postgres("-1 years -1months -1 days").unwrap();
        let interval_exp = Interval::new(-13, -1, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_18() {
        let interval = Interval::from_postgres("-1 years -1 months -1 days -1 hours").unwrap();
        let interval_exp = Interval::new(-13, -1, -3600000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_19() {
        let interval =
            Interval::from_postgres("-1 years -1 months -1days -1hours -10minutes").unwrap();
        let interval_exp = Interval::new(-13, -1, -4200000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_20() {
        let interval =
            Interval::from_postgres("-1years -1 mons -1 days -1hours -10minutes -15seconds")
                .unwrap();
        let interval_exp = Interval::new(-13, -1, -4215000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_21() {
        let interval = Interval::from_postgres("-1 hours").unwrap();
        let interval_exp = Interval::new(0, 0, -3600000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_22() {
        let interval = Interval::from_postgres("-1 hours -10minutes").unwrap();
        let interval_exp = Interval::new(0, 0, -4200000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_23() {
        let interval = Interval::from_postgres("-1 hours -10minutes -15 seconds").unwrap();
        let interval_exp = Interval::new(0, 0, -4215000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_24() {
        let interval = Interval::from_postgres("years 1");
        assert!(interval.is_err());
    }

    #[test]
    fn test_from_postgres_25() {
        let interval = Interval::from_postgres("- years");
        assert!(interval.is_err());
    }

    #[test]
    fn test_from_postgres_26() {
        let interval = Interval::from_postgres("10");
        assert!(interval.is_err());
    }

    #[test]
    fn test_from_postgres_27() {
        let interval = Interval::from_postgres("1.2 years").unwrap();
        let interval_exp = Interval::new(14, 0, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_28() {
        let interval = Interval::from_postgres("1.2 months").unwrap();
        let interval_exp = Interval::new(1, 6, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_29() {
        let interval = Interval::from_postgres("1.2 seconds").unwrap();
        let interval_exp = Interval::new(0, 0, 1_200_000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_postgres_30() {
        let interval = Interval::from_postgres("!");
        assert!(interval.is_err());
    }
}
