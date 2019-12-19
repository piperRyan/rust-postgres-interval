use crate::interval_norm::IntervalNorm;
use crate::pg_interval::Interval;
use super::parse_error::ParseError;
use super::{
    scale_date, scale_time, DAYS_PER_MONTH, HOURS_PER_DAY, MICROS_PER_SECOND, MINUTES_PER_HOUR,
    MONTHS_PER_YEAR, SECONDS_PER_MIN,
};

enum ParserCode {
    BADFORMAT,
    GOOD,
    DELIMFOUND,
}

impl Interval {
    pub fn from_iso<'a>(iso_str: &'a str) -> Result<Interval, ParseError> {
        let mut date_part = true;
        let delim = vec!['Y', 'M', 'D', 'H', 'S'];
        let mut number = "".to_owned();
        let mut interval_norm = IntervalNorm::default();
        if iso_str.rfind('P').map_or(false, |v| v == 1) {
            Err(ParseError::from_invalid_interval(
                "Invalid format must start with P.",
            ))
        } else if iso_str.len() < 2 {
            Err(ParseError::from_invalid_interval(
                "Invalid format length is less than 2.",
            ))
        } else {
            for x in iso_str.chars() {
                if x == 'P' {
                    continue;
                }
                if x == 'T' && date_part {
                    date_part = false;
                    continue;
                }
                let code = consume_number(&x, &mut number, &delim);
                match code {
                    ParserCode::BADFORMAT => {
                        return Err(ParseError::from_invalid_interval("Invalid format."));
                    }
                    ParserCode::GOOD => {
                        continue;
                    }
                    ParserCode::DELIMFOUND => {
                        let val = parse_number(&mut number)?;
                        match x {
                            'Y' => {
                                let (year, month) = scale_date(val, MONTHS_PER_YEAR);
                                interval_norm.years += year;
                                interval_norm.months += month;
                            }
                            'M' => {
                                if date_part {
                                    let (month, day) = scale_date(val, DAYS_PER_MONTH);
                                    interval_norm.months += month;
                                    interval_norm.days += day;
                                } else {
                                    let (minutes, seconds) = scale_time(val, SECONDS_PER_MIN);
                                    interval_norm.minutes += minutes;
                                    interval_norm.seconds += seconds;
                                }
                            }
                            'D' => {
                                let (days, hours) = scale_date(val, HOURS_PER_DAY);
                                interval_norm.days += days;
                                interval_norm.hours += hours as i64;
                            }
                            'H' => {
                                let (hours, minutes) = scale_time(val, MINUTES_PER_HOUR);
                                interval_norm.hours += hours;
                                interval_norm.minutes += minutes;
                            }
                            'S' => {
                                if date_part {
                                    return Err(ParseError::from_invalid_interval(
                                        "Cannot have S in date part.",
                                    ));
                                }
                                let (seconds, microseconds) = scale_time(val, MICROS_PER_SECOND);
                                interval_norm.seconds += seconds;
                                interval_norm.microseconds += microseconds;
                            }
                            _ => {
                                return Err(ParseError::from_invalid_interval(
                                    "Invalid format unknown delimiter.",
                                ));
                            }
                        }
                    }
                }
            }
            if number != "" {
                Err(ParseError::from_invalid_interval(
                    "Invalid format could not parse whole interval.",
                ))
            } else {
                interval_norm.try_into_interval()
            }
        }
    }
}

fn consume_number<'a>(val: &'a char, number: &'a mut String, delim: &'a Vec<char>) -> ParserCode {
    if val.is_digit(10) {
        number.push(*val);
        ParserCode::GOOD
    } else if number.len() == 0 && *val == '-' {
        number.push(*val);
        ParserCode::GOOD
    } else if number.len() != 0 && *val == '.' {
        number.push(*val);
        ParserCode::GOOD
    } else if delim.contains(&val) {
        ParserCode::DELIMFOUND
    } else {
        ParserCode::BADFORMAT
    }
}

fn parse_number<'a>(number: &'a mut String) -> Result<f64, ParseError> {
    let parse_num = number.parse::<f64>()?;
    if parse_num > i32::max_value() as f64 {
        Err(ParseError::from_invalid_interval("Exceeded max value"))
    } else {
        *number = "".to_owned();
        Ok(parse_num)
    }
}

#[cfg(test)]
mod tests {
    use crate::pg_interval::Interval;

    #[test]
    fn test_from_iso_1() {
        let interval = Interval::from_iso("P1Y").unwrap();
        let interval_exp = Interval::new(12, 0, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_2() {
        let interval = Interval::from_iso("P1Y1M").unwrap();
        let interval_exp = Interval::new(13, 0, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_3() {
        let interval = Interval::from_iso("P1Y1M1D").unwrap();
        let interval_exp = Interval::new(13, 1, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_4() {
        let interval = Interval::from_iso("P1Y1M1DT1H").unwrap();
        let interval_exp = Interval::new(13, 1, 3600000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_5() {
        let interval = Interval::from_iso("P1Y1M1DT1H10M").unwrap();
        let interval_exp = Interval::new(13, 1, 4200000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_6() {
        let interval = Interval::from_iso("P1Y1M1DT1H10M15S").unwrap();
        let interval_exp = Interval::new(13, 1, 4215000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_7() {
        let interval = Interval::from_iso("PT1H").unwrap();
        let interval_exp = Interval::new(0, 0, 3600000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_8() {
        let interval = Interval::from_iso("PT1H10M").unwrap();
        let interval_exp = Interval::new(0, 0, 4200000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_9() {
        let interval = Interval::from_iso("PT1H10M15S").unwrap();
        let interval_exp = Interval::new(0, 0, 4215000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_10() {
        let interval = Interval::from_iso("P-1Y").unwrap();
        let interval_exp = Interval::new(-12, 0, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_11() {
        let interval = Interval::from_iso("P-1Y-1M").unwrap();
        let interval_exp = Interval::new(-13, 0, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_12() {
        let interval = Interval::from_iso("P-1Y-1M-1D").unwrap();
        let interval_exp = Interval::new(-13, -1, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_13() {
        let interval = Interval::from_iso("P-1Y-1M-1DT-1H").unwrap();
        let interval_exp = Interval::new(-13, -1, -3600000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_14() {
        let interval = Interval::from_iso("P-1Y-1M-1DT-1H-10M").unwrap();
        let interval_exp = Interval::new(-13, -1, -4200000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_15() {
        let interval = Interval::from_iso("P-1Y-1M-1DT-1H-10M-15S").unwrap();
        let interval_exp = Interval::new(-13, -1, -4215000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_16() {
        let interval = Interval::from_iso("PT-1H").unwrap();
        let interval_exp = Interval::new(0, 0, -3600000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_17() {
        let interval = Interval::from_iso("PT-1H-10M").unwrap();
        let interval_exp = Interval::new(0, 0, -4200000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_18() {
        let interval = Interval::from_iso("PT-1H-10M-15S").unwrap();
        let interval_exp = Interval::new(0, 0, -4215000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_19() {
        let interval = Interval::from_iso("PTT");
        assert_eq!(interval.is_err(), true);
    }

    #[test]
    fn test_from_8601_20() {
        let interval = Interval::from_iso("PT-");
        assert_eq!(interval.is_err(), true);
    }

    #[test]
    fn test_from_8601_21() {
        let interval = Interval::from_iso("PT10");
        assert_eq!(interval.is_err(), true);
    }

    #[test]
    fn test_from_8601_22() {
        let interval = Interval::from_iso("P1.2YT0S").unwrap();
        let interval_exp = Interval::new(14, 0, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_23() {
        let interval = Interval::from_iso("P1.2MT0S").unwrap();
        let interval_exp = Interval::new(1, 6, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_24() {
        let interval = Interval::from_iso("PT1.2S").unwrap();
        let interval_exp = Interval::new(0, 0, 1_200_000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_25() {
        let interval = Interval::from_iso("PT5S5S").unwrap();
        let interval_exp = Interval::new(0, 0, 10000000);
        assert_eq!(interval, interval_exp);
    }

}
