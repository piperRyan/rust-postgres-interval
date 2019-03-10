use pg_interval::Interval;
use super::parse_error::ParseError;
use interval_norm::IntervalNorm;

static DAYS_PER_MONTH: i32 = 30;
static MONTHS_PER_YEAR: i32 = 12;
static SECONDS_PER_MIN: i32 = 60;
static HOURS_PER_DAY: i32 = 24;
static MINUTES_PER_HOUR: i32 = 60;
static MICROS_PER_SECOND: i32 = 1_000_000;


enum ParserCode {
    BADFORMAT, 
    GOOD,
    DELIMFOUND
}

impl Interval {
    pub fn from_iso<'a>(iso_str: &'a str) -> Result<Interval,ParseError> {
    let mut date_part = true;
    let delim = vec!('Y', 'M', 'D', 'H', 'S');
    let mut number = "".to_owned();
    let mut interval_norm = IntervalNorm::default();
    if iso_str.rfind('P').map_or(false, |v| v == 1) {
          Err(ParseError::from_invalid_interval("Invalid format must start with P."))
    } else if iso_str.len() < 2 {
          Err(ParseError::from_invalid_interval("Invalid format length is less than 2."))
    } else {
        for x in iso_str.chars() {
            if x == 'P' {
                continue;
            } 
            if x == 'T' {
                date_part = false;
                continue;
            }
            let code = consume_number(&x, &mut number, &delim); 
            match code {
                ParserCode::BADFORMAT => {
                    return Err(ParseError::from_invalid_interval("Invalid format."));
                },
                ParserCode::GOOD => {
                    continue;
                },  
                ParserCode::DELIMFOUND => {
                    let val = parse_number(&mut number)?;
                    match x {
                        'Y' => {
                            let (year, month) = scale_date(val, MONTHS_PER_YEAR);
                            interval_norm.years += year; 
                            interval_norm.months += month;
                        },
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
                        }, 
                        'D' => {
                            let (days, hours) = scale_date(val, HOURS_PER_DAY);
                            interval_norm.days += days; 
                            interval_norm.hours += hours as i64;
                        }, 
                        'H' => {
                            let (hours, minutes) = scale_time(val, MINUTES_PER_HOUR);
                            interval_norm.hours += hours; 
                            interval_norm.minutes += minutes;
                        }, 
                        'S' => {
                            let(seconds, microseconds) = scale_time(val, MICROS_PER_SECOND);
                            interval_norm.seconds += seconds; 
                            interval_norm.microseconds += microseconds; 
                        },
                        _ => {
                            return Err(ParseError::from_invalid_interval("Invalid format unknown delimiter."));
                        }
                    }
                }
            } 
        }
        interval_norm.try_into_interval()
      }
    }
}

fn consume_number<'a>(
    val: &'a char, 
    number: &'a mut String, 
    delim: &'a Vec<char>) -> ParserCode {
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

fn parse_number<'a> (number: &'a mut String) -> Result<f64, ParseError> {
    let parse_num =  number.parse::<f64>()?;
    if parse_num > i32::max_value() as f64 {
        Err(ParseError::from_invalid_interval("Exceeded max value"))
    } else {
        *number = "".to_owned();
        Ok(parse_num)
    }
}

fn scale_date(val: f64, scale: i32) -> (i32, i32) {
    if val.fract() == 0.0 {
        return (val.trunc() as i32, 0)
    } else {
        // matches postgres implementation of just truncating.
        let sub_value = (val.fract() * scale as f64) as i32;
        (val.trunc() as i32, sub_value)
    }
}

fn scale_time(val: f64, scale: i32) -> (i64, i64) {
      if val.fract() == 0.0 {
        return (val.trunc() as i64, 0)
    } else {
        // matches postgres implementation of just truncating.
        let sub_value = (val.fract() * scale as f64) as i64;
        (val.trunc() as i64, sub_value)
    }
} 


#[cfg(test)]
mod tests {
    use pg_interval::Interval;
    
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
        let interval_exp =  Interval::new(-12, 0, 0);
        assert_eq!(interval, interval_exp);
    }


    #[test]
    fn test_from_8601_11() {
        let interval = Interval::from_iso("P-1Y-1M").unwrap();
        let interval_exp =  Interval::new(-13, 0, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_12() {
        let interval = Interval::from_iso("P-1Y-1M-1D").unwrap();
        let interval_exp =  Interval::new(-13, -1, 0);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_13() {
        let interval = Interval::from_iso("P-1Y-1M-1DT-1H").unwrap();
        let interval_exp =  Interval::new(-13, -1, -3600000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_14() {
         let interval = Interval::from_iso("P-1Y-1M-1DT-1H-10M").unwrap();
        let interval_exp =  Interval::new(-13, -1, -4200000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_15() {
        let interval = Interval::from_iso("P-1Y-1M-1DT-1H-10M-15S").unwrap();
        let interval_exp =  Interval::new(-13, -1, -4215000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_16() {
        let interval = Interval::from_iso("PT-1H").unwrap();
        let interval_exp =  Interval::new(0, 0, -3600000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_17() {
        let interval = Interval::from_iso("PT-1H-10M").unwrap();
        let interval_exp =  Interval::new(0, 0, -4200000000);
        assert_eq!(interval, interval_exp);
    }

    #[test]
    fn test_from_8601_18() {
        let interval = Interval::from_iso("PT-1H-10M-15S").unwrap();
        let interval_exp =  Interval::new(0, 0, -4215000000);
        assert_eq!(interval, interval_exp);
    } 
}
