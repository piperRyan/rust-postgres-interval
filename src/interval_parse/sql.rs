use super::parse_error::ParseError;
use crate::interval_norm::IntervalNorm;
use crate::Interval;

impl Interval {
    pub fn from_sql(sql_str: &str) -> Result<Interval, ParseError> {
        if sql_str == "0" {
            return Ok(Interval::new(0, 0, 0));
        }

        let tokens: Vec<&str> = sql_str.split_whitespace().collect();
        let mut interval_norm = IntervalNorm::default();

        match tokens.len() {
            1 => {
                let token = tokens[0];
                if token.contains(':') {
                    parse_time_part(token, &mut interval_norm, true)?;
                } else if token.contains('-') {
                    parse_year_month_part(token, &mut interval_norm)?;
                } else {
                    return Err(ParseError::from_invalid_interval(
                        "Invalid format: expected year-month or time format",
                    ));
                }
            }
            2 => {
                parse_day_part(tokens[0], &mut interval_norm)?;
                parse_time_part(tokens[1], &mut interval_norm, true)?;
            }
            3 => {
                parse_year_month_part(tokens[0], &mut interval_norm)?;
                parse_day_part(tokens[1], &mut interval_norm)?;
                parse_time_part(tokens[2], &mut interval_norm, false)?;
            }
            _ => {
                return Err(ParseError::from_invalid_interval(
                    "Invalid format: expected 1-3 tokens",
                ));
            }
        }

        interval_norm.try_into_interval()
    }
}

fn parse_year_month_part(token: &str, interval: &mut IntervalNorm) -> Result<(), ParseError> {
    let token = token.trim_start_matches('+');

    let (sign, rest) = if let Some(stripped) = token.strip_prefix('-') {
        (-1, stripped)
    } else {
        (1, token)
    };

    let (years_str, months_str) = if let Some(pos) = rest.find('-') {
        (&rest[..pos], &rest[pos + 1..])
    } else {
        return Err(ParseError::from_invalid_interval(
            "Invalid year-month format",
        ));
    };

    let years = if years_str.is_empty() {
        0
    } else {
        years_str.parse::<i32>()? * sign
    };

    let months: i32 = months_str.parse::<i32>()? * sign;

    interval.years = years;
    interval.months = months;
    Ok(())
}

fn parse_day_part(token: &str, interval: &mut IntervalNorm) -> Result<(), ParseError> {
    let days: i32 = token.parse()?;
    interval.days = days;
    Ok(())
}

fn parse_time_part(
    token: &str,
    interval: &mut IntervalNorm,
    is_only_time: bool,
) -> Result<(), ParseError> {
    let (time_token, sign) = if is_only_time {
        let is_negative = token.starts_with('-');
        let token_str = if is_negative { &token[1..] } else { token };
        (token_str, if is_negative { -1 } else { 1 })
    } else {
        let is_negative = token.starts_with('-');
        let token_str = token.trim_start_matches('+').trim_start_matches('-');
        (token_str, if is_negative { -1 } else { 1 })
    };

    let time_parts: Vec<&str> = time_token.split(':').collect();

    if time_parts.len() < 2 || time_parts.len() > 3 {
        return Err(ParseError::from_invalid_interval("Invalid time format"));
    }

    let hours: i64 = time_parts[0].parse()?;
    let minutes: i64 = time_parts[1].parse()?;

    let (seconds, microseconds) = if time_parts.len() == 3 {
        parse_seconds_part(time_parts[2])?
    } else {
        (0, 0)
    };

    interval.hours = hours * sign;
    interval.minutes = minutes * sign;
    interval.seconds = seconds * sign;
    interval.microseconds = microseconds * sign;
    Ok(())
}

fn parse_seconds_part(token: &str) -> Result<(i64, i64), ParseError> {
    if token.contains('.') {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 2 {
            return Err(ParseError::from_invalid_interval("Invalid seconds format"));
        }
        let seconds: i64 = parts[0].parse()?;
        let mut micros_str = parts[1].to_string();

        if micros_str.len() > 6 {
            return Err(ParseError::from_invalid_interval(
                "Microseconds precision too high",
            ));
        }

        while micros_str.len() < 6 {
            micros_str.push('0');
        }

        let microseconds: i64 = micros_str.parse()?;
        Ok((seconds, microseconds))
    } else {
        let seconds: i64 = token.parse()?;
        Ok((seconds, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_sql_1() {
        let interval = Interval::from_sql("1-0").unwrap();
        let interval_exp = Interval::new(12, 0, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_sql_2() {
        let interval = Interval::from_sql("1-1").unwrap();
        let interval_exp = Interval::new(13, 0, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_sql_3() {
        let interval = Interval::from_sql("+1-1 +1 +0:00:00").unwrap();
        let interval_exp = Interval::new(13, 1, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_sql_4() {
        let interval = Interval::from_sql("+1-1 +1 +1:00:00").unwrap();
        let interval_exp = Interval::new(13, 1, 3600000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_sql_5() {
        let interval = Interval::from_sql("+1-1 +1 +1:10:00").unwrap();
        let interval_exp = Interval::new(13, 1, 4200000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_sql_6() {
        let interval = Interval::from_sql("+1-1 +1 +1:10:15").unwrap();
        let interval_exp = Interval::new(13, 1, 4215000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_sql_7() {
        let interval = Interval::from_sql("1:00:00").unwrap();
        let interval_exp = Interval::new(0, 0, 3600000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_sql_8() {
        let interval = Interval::from_sql("1:10:00").unwrap();
        let interval_exp = Interval::new(0, 0, 4200000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_sql_9() {
        let interval = Interval::from_sql("1:10:15").unwrap();
        let interval_exp = Interval::new(0, 0, 4215000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_sql_10() {
        let interval = Interval::from_sql("-1-0").unwrap();
        let interval_exp = Interval::new(-12, 0, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_sql_11() {
        let interval = Interval::from_sql("-1-1").unwrap();
        let interval_exp = Interval::new(-13, 0, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_sql_12() {
        let interval = Interval::from_sql("-1-1 -1 +0:00:00").unwrap();
        let interval_exp = Interval::new(-13, -1, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_sql_13() {
        let interval = Interval::from_sql("-1-1 -1 -1:00:00").unwrap();
        let interval_exp = Interval::new(-13, -1, -3600000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_sql_14() {
        let interval = Interval::from_sql("-1-1 -1 -1:10:00").unwrap();
        let interval_exp = Interval::new(-13, -1, -4200000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_sql_15() {
        let interval = Interval::from_sql("-1-1 -1 -1:10:15").unwrap();
        let interval_exp = Interval::new(-13, -1, -4215000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_sql_16() {
        let interval = Interval::from_sql("-1:00:00").unwrap();
        let interval_exp = Interval::new(0, 0, -3600000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_sql_17() {
        let interval = Interval::from_sql("-1:10:00").unwrap();
        let interval_exp = Interval::new(0, 0, -4200000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_sql_18() {
        let interval = Interval::from_sql("-1:10:15").unwrap();
        let interval_exp = Interval::new(0, 0, -4215000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_sql_19() {
        let interval = Interval::from_sql("0").unwrap();
        let interval_exp = Interval::new(0, 0, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_sql_20() {
        let interval = Interval::from_sql("invalid");
        assert!(interval.is_err());
    }

    #[test]
    fn test_roundtrip_sql_1() {
        let original = Interval::new(12, 0, 0);
        let sql = original.to_sql();
        let parsed = Interval::from_sql(&sql).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_roundtrip_sql_2() {
        let original = Interval::new(13, 0, 0);
        let sql = original.to_sql();
        let parsed = Interval::from_sql(&sql).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_roundtrip_sql_3() {
        let original = Interval::new(13, 1, 0);
        let sql = original.to_sql();
        let parsed = Interval::from_sql(&sql).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_roundtrip_sql_4() {
        let original = Interval::new(13, 1, 3600000000);
        let sql = original.to_sql();
        let parsed = Interval::from_sql(&sql).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_roundtrip_sql_5() {
        let original = Interval::new(13, 1, 4200000000);
        let sql = original.to_sql();
        let parsed = Interval::from_sql(&sql).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_roundtrip_sql_6() {
        let original = Interval::new(13, 1, 4215000000);
        let sql = original.to_sql();
        let parsed = Interval::from_sql(&sql).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_roundtrip_sql_7() {
        let original = Interval::new(0, 0, 3600000000);
        let sql = original.to_sql();
        let parsed = Interval::from_sql(&sql).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_roundtrip_sql_8() {
        let original = Interval::new(0, 0, 4200000000);
        let sql = original.to_sql();
        let parsed = Interval::from_sql(&sql).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_roundtrip_sql_9() {
        let original = Interval::new(0, 0, 4215000000);
        let sql = original.to_sql();
        let parsed = Interval::from_sql(&sql).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_roundtrip_sql_10() {
        let original = Interval::new(-12, 0, 0);
        let sql = original.to_sql();
        let parsed = Interval::from_sql(&sql).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_roundtrip_sql_11() {
        let original = Interval::new(-13, 0, 0);
        let sql = original.to_sql();
        let parsed = Interval::from_sql(&sql).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_roundtrip_sql_12() {
        let original = Interval::new(-13, -1, 0);
        let sql = original.to_sql();
        let parsed = Interval::from_sql(&sql).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_roundtrip_sql_13() {
        let original = Interval::new(-13, -1, -3600000000);
        let sql = original.to_sql();
        let parsed = Interval::from_sql(&sql).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_roundtrip_sql_14() {
        let original = Interval::new(-13, -1, -4200000000);
        let sql = original.to_sql();
        let parsed = Interval::from_sql(&sql).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_roundtrip_sql_15() {
        let original = Interval::new(-13, -1, -4215000000);
        let sql = original.to_sql();
        let parsed = Interval::from_sql(&sql).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_roundtrip_sql_16() {
        let original = Interval::new(0, 0, -3600000000);
        let sql = original.to_sql();
        let parsed = Interval::from_sql(&sql).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_roundtrip_sql_17() {
        let original = Interval::new(0, 0, -4200000000);
        let sql = original.to_sql();
        let parsed = Interval::from_sql(&sql).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_roundtrip_sql_18() {
        let original = Interval::new(0, 0, -4215000000);
        let sql = original.to_sql();
        let parsed = Interval::from_sql(&sql).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_roundtrip_sql_19() {
        let original = Interval::new(0, 0, 0);
        let sql = original.to_sql();
        let parsed = Interval::from_sql(&sql).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_roundtrip_sql_with_microseconds() {
        let original = Interval::new(1, 2, 123456);
        let sql = original.to_sql();
        let parsed = Interval::from_sql(&sql).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_postgres_compatibility_year_month_only_positive() {
        let sql = "1-0";
        let interval = Interval::from_sql(sql).unwrap();
        assert_eq!(interval, Interval::new(12, 0, 0));
    }

    #[test]
    fn test_postgres_compatibility_year_month_only_negative() {
        let sql = "-1-1";
        let interval = Interval::from_sql(sql).unwrap();
        assert_eq!(interval, Interval::new(-13, 0, 0));
    }

    #[test]
    fn test_postgres_compatibility_time_only_positive() {
        let sql = "1:00:00";
        let interval = Interval::from_sql(sql).unwrap();
        assert_eq!(interval, Interval::new(0, 0, 3600000000));
    }

    #[test]
    fn test_postgres_compatibility_time_only_negative() {
        let sql = "-1:10:15";
        let interval = Interval::from_sql(sql).unwrap();
        assert_eq!(interval, Interval::new(0, 0, -4215000000));
    }

    #[test]
    fn test_postgres_compatibility_full_interval_positive() {
        let sql = "+1-1 +1 +1:10:15";
        let interval = Interval::from_sql(sql).unwrap();
        assert_eq!(interval, Interval::new(13, 1, 4215000000));
    }

    #[test]
    fn test_postgres_compatibility_full_interval_negative() {
        let sql = "-1-1 -1 -1:10:15";
        let interval = Interval::from_sql(sql).unwrap();
        assert_eq!(interval, Interval::new(-13, -1, -4215000000));
    }

    #[test]
    fn test_postgres_compatibility_fractional_seconds() {
        let sql = "1:10:15.123456";
        let interval = Interval::from_sql(sql).unwrap();
        assert_eq!(interval, Interval::new(0, 0, 4215123456));
    }

    #[test]
    fn test_postgres_compatibility_half_second() {
        let sql = "1:00:00.5";
        let interval = Interval::from_sql(sql).unwrap();
        assert_eq!(interval, Interval::new(0, 0, 3600500000));
    }

    #[test]
    fn test_postgres_compatibility_full_fractional_interval() {
        let sql = "+1-1 +1 +1:10:15.123456";
        let interval = Interval::from_sql(sql).unwrap();
        assert_eq!(interval, Interval::new(13, 1, 4215123456));
    }

    #[test]
    fn test_roundtrip_fractional_seconds() {
        let original = Interval::new(0, 0, 4215123456);
        let sql = original.to_sql();
        let parsed = Interval::from_sql(&sql).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_roundtrip_fractional_with_months_days() {
        let original = Interval::new(13, 1, 4215123456);
        let sql = original.to_sql();
        let parsed = Interval::from_sql(&sql).unwrap();
        assert_eq!(original, parsed);
    }
}
