use super::parse_error::ParseError;
use crate::{interval_norm::IntervalNorm, Interval};

use super::{
    scale_date, scale_time, DAYS_PER_MONTH, HOURS_PER_DAY, MICROS_PER_SECOND, MINUTES_PER_HOUR,
    MONTHS_PER_YEAR, SECONDS_PER_MIN,
};

impl Interval {
    pub fn from_postgres(iso_str: &str) -> Result<Interval, ParseError> {
        let mut delim = vec![
            "years", "year", "months", "mons", "mon", "days", "day", "hours", "hour", "minutes",
            "minute", "seconds", "second",
        ];
        let mut time_tokens = iso_str.split(' ').collect::<Vec<&str>>(); // clean up empty values caused by n spaces between values.
        time_tokens.retain(|&token| !token.is_empty());
        // since there might not be space between the delim and the
        // value we need to scan each token.
        let mut final_tokens = Vec::with_capacity(time_tokens.len());
        for token in time_tokens {
            if is_token_time_format(token)? {
                let (hours, minutes, seconds, microseconds) = parse_time_format(token)?;
                if hours != 0 {
                    final_tokens.push(hours.to_string());
                    final_tokens.push("hours".to_owned());
                }
                if minutes != 0 {
                    final_tokens.push(minutes.to_string());
                    final_tokens.push("minutes".to_owned());
                }
                if seconds != 0 || microseconds != 0 {
                    let total_seconds =
                        seconds as f64 + microseconds as f64 / MICROS_PER_SECOND as f64;
                    final_tokens.push(total_seconds.to_string());
                    final_tokens.push("seconds".to_owned());
                }
            } else if is_token_alphanumeric(token)? {
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

/// Check if the token is a time format (e.g., "01:02:03" or "-01:02:03.123456")
fn is_token_time_format(val: &str) -> Result<bool, ParseError> {
    if !val.contains(':') {
        return Ok(false);
    }

    let parts: Vec<&str> = val.split(':').collect();
    if parts.len() != 3 {
        return Err(ParseError::from_invalid_interval("Invalid time format."));
    }

    for (i, part) in parts.iter().enumerate() {
        let is_first = i == 0;
        for character in part.chars() {
            if character.is_numeric() || (is_first && character == '-') {
                // OK
            } else if character == '.' && i == 2 {
                // Fractional seconds only in the last part
                continue;
            } else {
                return Err(ParseError::from_invalid_interval(
                    "Invalid character in time format.",
                ));
            }
        }
    }

    Ok(true)
}

/// Parse a time format token (e.g., "01:02:03" or "-01:02:03.123456")
/// Returns (hours, minutes, seconds, microseconds)
fn parse_time_format(val: &str) -> Result<(i64, i64, i64, i64), ParseError> {
    let is_negative = val.starts_with('-');
    let time_str = if is_negative { &val[1..] } else { val };

    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() != 3 {
        return Err(ParseError::from_invalid_interval("Invalid time format."));
    }

    let hours: i64 = parts[0]
        .parse()
        .map_err(|_| ParseError::from_invalid_interval("Invalid hours value."))?;

    let minutes: i64 = parts[1]
        .parse()
        .map_err(|_| ParseError::from_invalid_interval("Invalid minutes value."))?;

    let seconds_str = parts[2];
    let (seconds, microseconds) = if let Some(dot_pos) = seconds_str.find('.') {
        let whole_part = &seconds_str[..dot_pos];
        let fraction_part = &seconds_str[dot_pos + 1..];

        let seconds: i64 = whole_part
            .parse()
            .map_err(|_| ParseError::from_invalid_interval("Invalid seconds value."))?;

        let microseconds: i64 = if !fraction_part.is_empty() {
            let padded = format!("{:0<6}", fraction_part);
            padded[..6].parse().unwrap_or(0)
        } else {
            0
        };

        (seconds, microseconds)
    } else {
        let seconds: i64 = seconds_str
            .parse()
            .map_err(|_| ParseError::from_invalid_interval("Invalid seconds value."))?;
        (seconds, 0)
    };

    if is_negative {
        Ok((-hours, -minutes, -seconds, -microseconds))
    } else {
        Ok((hours, minutes, seconds, microseconds))
    }
}

/// Consume the token parts and add to the normalized interval.
fn consume_token<'a>(
    interval: &mut IntervalNorm,
    val: f64,
    delim: String,
    delim_list: &mut Vec<&'a str>,
) -> Result<(), ParseError> {
    // Unlike iso8601 the delimiter can only appear once
    // so we need to check if the token can be found in
    // the deliminator list.
    if delim_list.contains(&&*delim) {
        match &*delim {
            "years" | "year" => {
                let (year, month) = scale_date(val, MONTHS_PER_YEAR);
                interval.years += year;
                interval.months += month;
                delim_list.retain(|x| *x != "years" && *x != "year");
                Ok(())
            }
            "months" | "mons" | "mon" => {
                let (month, day) = scale_date(val, DAYS_PER_MONTH);
                interval.months += month;
                interval.days += day;
                delim_list.retain(|x| *x != "months" && *x != "mons" && *x != "mon");
                Ok(())
            }
            "days" | "day" => {
                let (days, hours) = scale_date(val, HOURS_PER_DAY);
                interval.days += days;
                interval.hours += hours as i64;
                delim_list.retain(|x| *x != "days" && *x != "day");
                Ok(())
            }
            "hours" | "hour" => {
                let (hours, minutes) = scale_time(val, MINUTES_PER_HOUR);
                interval.hours += hours;
                interval.minutes += minutes;
                delim_list.retain(|x| *x != "hours" && *x != "hour");
                Ok(())
            }
            "minutes" | "minute" => {
                let (minutes, seconds) = scale_time(val, SECONDS_PER_MIN);
                interval.minutes += minutes;
                interval.seconds += seconds;
                delim_list.retain(|x| *x != "minutes" && *x != "minute");
                Ok(())
            }
            "seconds" | "second" => {
                let (seconds, microseconds) = scale_time(val, MICROS_PER_SECOND);
                interval.seconds += seconds;
                interval.microseconds += microseconds;
                delim_list.retain(|x| *x != "seconds" && *x != "second");
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
        assert_eq!(interval.is_err(), true);
    }

    #[test]
    fn test_from_postgres_25() {
        let interval = Interval::from_postgres("- years");
        assert_eq!(interval.is_err(), true);
    }

    #[test]
    fn test_from_postgres_26() {
        let interval = Interval::from_postgres("10");
        assert_eq!(interval.is_err(), true);
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
        assert_eq!(interval.is_err(), true);
    }

    #[test]
    fn test_roundtrip_issue_time_with_colons() {
        let original = Interval::new(0, 0, 3661000000);
        let postgres_str = original.to_postgres();
        let result = Interval::from_postgres(&postgres_str);
        assert_eq!(
            result.unwrap(),
            original,
            "Time format with colons should roundtrip successfully: {}",
            postgres_str
        );
    }

    #[test]
    fn test_roundtrip_issue_12_plus_months() {
        let original = Interval::new(12, 0, 0);
        let postgres_str = original.to_postgres();
        let result = Interval::from_postgres(&postgres_str);
        assert_eq!(
            result.unwrap(),
            original,
            "Singular 'year' format should roundtrip successfully: {}",
            postgres_str
        );
    }

    #[test]
    fn test_roundtrip_issue_single_day() {
        let original = Interval::new(0, 1, 0);
        let postgres_str = original.to_postgres();
        let result = Interval::from_postgres(&postgres_str);
        assert_eq!(
            result.unwrap(),
            original,
            "Singular 'day' format should roundtrip successfully: {}",
            postgres_str
        );
    }

    #[test]
    fn test_roundtrip_issue_zero_interval() {
        let original = Interval::new(0, 0, 0);
        let postgres_str = original.to_postgres();
        let result = Interval::from_postgres(&postgres_str);
        assert_eq!(
            result.unwrap(),
            original,
            "Zero interval '00:00:00' should roundtrip successfully: {}",
            postgres_str
        );
    }

    #[test]
    fn test_roundtrip_issue_microseconds() {
        let original = Interval::new(0, 0, 1000000);
        let postgres_str = original.to_postgres();
        let result = Interval::from_postgres(&postgres_str);
        assert_eq!(
            result.unwrap(),
            original,
            "Microseconds format with colons should roundtrip successfully: {}",
            postgres_str
        );
    }

    #[test]
    fn test_to_postgres_matches_postgresql_12_months() {
        let interval = Interval::new(12, 0, 0);
        let lib_output = interval.to_postgres();
        assert_eq!(lib_output, "1 year");
        assert_eq!(lib_output, "1 year");
    }

    #[test]
    fn test_to_postgres_matches_postgresql_13_months() {
        let interval = Interval::new(13, 0, 0);
        let lib_output = interval.to_postgres();
        assert_eq!(lib_output, "1 year 1 mon");
    }

    #[test]
    fn test_to_postgres_matches_postgresql_1_day() {
        let interval = Interval::new(0, 1, 0);
        let lib_output = interval.to_postgres();
        assert_eq!(lib_output, "1 day");
    }

    #[test]
    fn test_to_postgres_matches_postgresql_zero() {
        let interval = Interval::new(0, 0, 0);
        let lib_output = interval.to_postgres();
        assert_eq!(lib_output, "00:00:00");
    }

    #[test]
    fn test_to_postgres_matches_postgresql_01_01_01() {
        let interval = Interval::new(0, 0, 3661000000);
        let lib_output = interval.to_postgres();
        assert_eq!(lib_output, "01:01:01");
    }

    #[test]
    fn test_to_postgres_matches_postgresql_1_second() {
        let interval = Interval::new(0, 0, 1000000);
        let lib_output = interval.to_postgres();
        assert_eq!(lib_output, "00:00:01");
    }

    #[test]
    fn test_postgresql_can_parse_singular_year() {
        assert!(
            Interval::from_postgres("1 year").is_ok(),
            "Library can now parse '1 year' like PostgreSQL can"
        );
    }

    #[test]
    fn test_postgresql_can_parse_singular_mon() {
        assert!(
            Interval::from_postgres("1 mon").is_ok(),
            "Library can now parse '1 mon' like PostgreSQL can"
        );
    }

    #[test]
    fn test_postgresql_can_parse_singular_day() {
        assert!(
            Interval::from_postgres("1 day").is_ok(),
            "Library can now parse '1 day' like PostgreSQL can"
        );
    }

    #[test]
    fn test_postgresql_can_parse_time_format_00_00_00() {
        assert!(
            Interval::from_postgres("00:00:00").is_ok(),
            "Library can now parse '00:00:00' time format like PostgreSQL can"
        );
    }

    #[test]
    fn test_postgresql_can_parse_time_format_01_01_01() {
        assert!(
            Interval::from_postgres("01:01:01").is_ok(),
            "Library can now parse '01:01:01' time format like PostgreSQL can"
        );
    }

    #[test]
    fn test_parse_time_format_with_microseconds() {
        let interval = Interval::from_postgres("01:01:01.123456").unwrap();
        assert_eq!(interval, Interval::new(0, 0, 3661123456));
    }

    #[test]
    fn test_parse_time_format_negative() {
        let interval = Interval::from_postgres("-01:01:01").unwrap();
        assert_eq!(interval, Interval::new(0, 0, -3661000000));
    }

    #[test]
    fn test_parse_singular_hour() {
        let interval = Interval::from_postgres("1 hour").unwrap();
        assert_eq!(interval, Interval::new(0, 0, 3600000000));
    }

    #[test]
    fn test_parse_singular_minute() {
        let interval = Interval::from_postgres("1 minute").unwrap();
        assert_eq!(interval, Interval::new(0, 0, 60000000));
    }

    #[test]
    fn test_parse_singular_second() {
        let interval = Interval::from_postgres("1 second").unwrap();
        assert_eq!(interval, Interval::new(0, 0, 1000000));
    }

    #[test]
    fn test_roundtrip_complex_interval() {
        let original = Interval::new(12, 1, 3661123456);
        let postgres_str = original.to_postgres();
        let result = Interval::from_postgres(&postgres_str);
        assert_eq!(
            result.unwrap(),
            original,
            "Complex interval should roundtrip successfully: {}",
            postgres_str
        );
    }

    #[test]
    fn test_roundtrip_all_formats() {
        let test_cases = vec![
            ("1 year", Interval::new(12, 0, 0)),
            ("1 mon", Interval::new(1, 0, 0)),
            ("1 day", Interval::new(0, 1, 0)),
            ("00:00:00", Interval::new(0, 0, 0)),
            ("01:01:01", Interval::new(0, 0, 3661000000)),
            ("01:01:01.123456", Interval::new(0, 0, 3661123456)),
            ("1 hour", Interval::new(0, 0, 3600000000)),
            ("1 minute", Interval::new(0, 0, 60000000)),
            ("1 second", Interval::new(0, 0, 1000000)),
        ];

        for (input, expected) in test_cases {
            let result = Interval::from_postgres(input).unwrap();
            assert_eq!(result, expected, "Failed to parse '{}'", input);
        }
    }

    #[test]
    fn test_to_postgres_from_postgres_roundtrip() {
        let intervals = vec![
            Interval::new(12, 0, 0),
            Interval::new(1, 0, 0),
            Interval::new(0, 1, 0),
            Interval::new(0, 0, 0),
            Interval::new(0, 0, 3661000000),
            Interval::new(0, 0, 3661123456),
            Interval::new(0, 0, 3600000000),
            Interval::new(0, 0, 60000000),
            Interval::new(0, 0, 1000000),
            Interval::new(12, 1, 3661123456),
        ];

        for original in intervals {
            let postgres_str = original.to_postgres();
            let result = Interval::from_postgres(&postgres_str).unwrap();
            assert_eq!(
                result, original,
                "Roundtrip failed for {:?} -> {} -> {:?}",
                original, postgres_str, result
            );
        }
    }

    #[test]
    fn test_parse_edge_case_mixed_singular_plural() {
        let interval = Interval::from_postgres("1 year 2 mons").unwrap();
        assert_eq!(interval, Interval::new(14, 0, 0));
    }

    #[test]
    fn test_parse_zero_values() {
        assert_eq!(
            Interval::from_postgres("0 year").unwrap(),
            Interval::new(0, 0, 0)
        );
        assert_eq!(
            Interval::from_postgres("0 mon").unwrap(),
            Interval::new(0, 0, 0)
        );
        assert_eq!(
            Interval::from_postgres("0 day").unwrap(),
            Interval::new(0, 0, 0)
        );
    }

    #[test]
    fn test_parse_single_digit_time() {
        let interval = Interval::from_postgres("1:00:00").unwrap();
        assert_eq!(interval, Interval::new(0, 0, 3600000000));
    }

    #[test]
    fn test_parse_fractional_values() {
        assert_eq!(
            Interval::from_postgres("1.5 years").unwrap(),
            Interval::new(18, 0, 0)
        );
        assert_eq!(
            Interval::from_postgres("1.5 hours").unwrap(),
            Interval::new(0, 0, 5400000000)
        );
        assert_eq!(
            Interval::from_postgres("1.5 seconds").unwrap(),
            Interval::new(0, 0, 1500000)
        );
    }

    #[test]
    fn test_parse_all_units() {
        let interval = Interval::from_postgres("1 days 2 hours 3 minutes 4.567 seconds").unwrap();
        assert_eq!(interval, Interval::new(0, 1, 7384567000));
    }

    #[test]
    fn test_roundtrip_comprehensive() {
        let test_cases = vec![
            Interval::new(18, 0, 0),
            Interval::new(0, 0, 5400000000),
            Interval::new(0, 0, 1500000),
            Interval::new(0, 1, 7384567000),
            Interval::new(13, -1, 0),
            Interval::new(-13, 1, 0),
            Interval::new(-12, 0, 0),
            Interval::new(0, -1, 0),
            Interval::new(0, 0, -3600000000),
        ];

        for original in test_cases {
            let postgres_str = original.to_postgres();
            let result = Interval::from_postgres(&postgres_str).unwrap();
            assert_eq!(
                result, original,
                "Roundtrip failed for {:?} -> {} -> {:?}",
                original, postgres_str, result
            );
        }
    }

    #[test]
    fn test_negative_values_postgresql_format() {
        assert_eq!(Interval::new(-12, 0, 0).to_postgres(), "-1 years");
        assert_eq!(Interval::new(0, -1, 0).to_postgres(), "-1 days");
        assert_eq!(Interval::new(0, 0, -3600000000).to_postgres(), "-01:00:00");

        assert_eq!(
            Interval::from_postgres("-1 years").unwrap(),
            Interval::new(-12, 0, 0)
        );
        assert_eq!(
            Interval::from_postgres("-1 days").unwrap(),
            Interval::new(0, -1, 0)
        );
        assert_eq!(
            Interval::from_postgres("-01:00:00").unwrap(),
            Interval::new(0, 0, -3600000000)
        );
    }

    #[test]
    fn test_large_values() {
        assert_eq!(
            Interval::from_postgres("9999 years").unwrap(),
            Interval::new(9999 * 12, 0, 0)
        );
        assert_eq!(
            Interval::from_postgres("10000 years").unwrap(),
            Interval::new(10000 * 12, 0, 0)
        );
    }

    #[test]
    fn test_time_format_with_leading_zeros() {
        assert_eq!(
            Interval::from_postgres("01:00:00").unwrap(),
            Interval::new(0, 0, 3600000000)
        );
        assert_eq!(
            Interval::from_postgres("00:01:00").unwrap(),
            Interval::new(0, 0, 60000000)
        );
        assert_eq!(
            Interval::from_postgres("00:00:01").unwrap(),
            Interval::new(0, 0, 1000000)
        );
    }
}
