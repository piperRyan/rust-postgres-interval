// Helper function to help derive the year month interval for a iso-8601
// compliant string.
pub fn get_year_month_interval(years: i32, months: i32, days: i32) -> Option<String> {
    if years != 0 && months != 0  && days != 0 {
        Some(format!("{:#?} year {:#?} mons {:#?} days", years, months, days))
    } else if years != 0 && months != 0 && days == 0 {
        Some(format!("{:#?} year {:#?} mons", years, months))
    } else if years != 0 && months == 0 && days == 0 {
        Some(format!("{:#?} year", years))
    } else if years == 0 && months != 0 && days != 0 {
        Some(format!("{:#?} mons {:#?} days", months, days))
    } else if years == 0 && months != 0 && days == 0  {
        Some(format!("{:#?} mons", months))
    } else if years == 0 && months == 0 && days != 0 {
        Some(format!("{:#?} days", days))
    } else if years != 0 && months == 0 && days != 0 {
        Some(format!("{:#?} year {:#?} days", years, days))
    } else {
        None
    }
}

// Helper function to help derive the day-time interval for a iso-8601
// compliant string.
pub fn get_day_time_interval(hours: i64, minutes: i64, seconds: f64) -> String {
    let fmt_seconds = |secs: f64 | -> String {
        let secs = secs.abs();
        if secs < 10.0 && secs > -10.0 {
            let mut seconds_fmt = "0".to_owned();
             seconds_fmt +=  &*secs.to_string();
             seconds_fmt
        } else {
            secs.to_string()
        }
    };
    let fmt_min_or_hours = | unit: i64 | -> String {
        let unit = unit.abs();
        if unit < 10 && unit > -10 {
            let mut unit_fmt = "0".to_owned();
            unit_fmt += &*unit.to_string();
            unit_fmt
        } else {
            unit.to_string()
        }
    };
    let hours_fmt = fmt_min_or_hours(hours);
    let minutes_fmt = fmt_min_or_hours(minutes);
    let seconds_fmt = fmt_seconds(seconds);
    if hours < 0 || minutes < 0 || seconds < 0.0 {
        "-".to_owned() + &*hours_fmt  + ":" + &*minutes_fmt + ":" + &*seconds_fmt
    } else {
        hours_fmt + ":" + &*minutes_fmt + ":" + &*seconds_fmt
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_year_month_interval_1() {
        let year: i32 =1;
        let months: i32 =  2;
        let days: i32 = 21;
        let interval = super::get_year_month_interval(year, months, days);
        assert_eq!(Some(String::from("1 year 2 mons 21 days")), interval);
    }

    #[test]
    fn test_get_year_month_interval_2() {
        let year: i32 =0;
        let months: i32 =  2;
        let days: i32 = 21;
        let interval = super::get_year_month_interval(year, months, days);
        assert_eq!(Some(String::from("2 mons 21 days")), interval);
    }

    #[test]
    fn test_get_year_month_interval_3() {
        let year: i32 =0;
        let months: i32 =  0;
        let days: i32 = 21;
        let interval = super::get_year_month_interval(year, months, days);
        assert_eq!(Some(String::from("21 days")), interval);
    }

    #[test]
    fn test_get_year_month_interval_4() {
        let year: i32 =0;
        let months: i32 =  0;
        let days: i32 = 0;
        let interval = super::get_year_month_interval(year, months, days);
        assert_eq!(None, interval);
    }

    #[test]
    fn test_get_year_month_interval_5() {
        let year: i32 =1;
        let months: i32 =  11;
        let days: i32 = 0;
        let interval = super::get_year_month_interval(year, months, days);
        assert_eq!(Some(String::from("1 year 11 mons")), interval);
    }

    #[test]
    fn test_get_year_month_interval_6() {
        let year: i32 =1;
        let months: i32 =  0;
        let days: i32 = 21;
        let interval = super::get_year_month_interval(year, months, days);
        assert_eq!(Some(String::from("1 year 21 days")), interval);
    }

    #[test]
    fn test_get_year_month_interval_7() {
        let year: i32 =1;
        let months: i32 =  0;
        let days: i32 = 0;
        let interval = super::get_year_month_interval(year, months, days);
        assert_eq!(Some(String::from("1 year")), interval);
    }

    #[test]
    fn test_get_year_month_interval_8() {
        let year: i32 =0;
        let months: i32 =  1;
        let days: i32 = 0;
        let interval = super::get_year_month_interval(year, months, days);
        assert_eq!(Some(String::from("1 mons")), interval);
    }

    #[test]
    fn test_get_day_time_interval_1() {
        let hour: i64 = 1;
        let minutes: i64 =  1;
        let seconds: f64 = 1.25;
        let interval = super::get_day_time_interval(hour,minutes,seconds);
        assert_eq!(String::from("01:01:01.25"), interval);
    }

    #[test]
    fn test_get_day_time_interval_2() {
        let hour: i64 = 1;
        let minutes: i64 =  1;
        let seconds: f64 = 1.0;
        let interval = super::get_day_time_interval(hour,minutes,seconds);
        assert_eq!(String::from("01:01:01"), interval);
    }

    #[test]
    fn test_get_day_time_interval_3() {
        let hour: i64 = 1;
        let minutes: i64 =  1;
        let seconds: f64 = 0.0;
        let interval = super::get_day_time_interval(hour,minutes,seconds);
        assert_eq!(String::from("01:01:00"), interval);
    }

    #[test]
    fn test_get_day_time_interval_4() {
        let hour: i64 = 1;
        let minutes: i64 =  0;
        let seconds: f64 = 1.24;
        let interval = super::get_day_time_interval(hour,minutes,seconds);
        assert_eq!(String::from("01:00:01.24"), interval);
    }

    #[test]
    fn test_get_day_time_interval_5() {
        let hour: i64 = 0;
        let minutes: i64 =  1;
        let seconds: f64 = 1.24;
        let interval = super::get_day_time_interval(hour,minutes,seconds);
        assert_eq!(String::from("00:01:01.24"), interval);
    }

    #[test]
    fn test_get_day_time_interval_6() {
        let hour: i64 = 10;
        let minutes: i64 =  0;
        let seconds: f64 = 0.0;
        let interval = super::get_day_time_interval(hour,minutes,seconds);
        assert_eq!(String::from("10:00:00"), interval);
    }

    #[test]
    fn test_get_day_time_interval_7() {
        let hour: i64 = 0;
        let minutes: i64 =  1;
        let seconds: f64 = 0.0;
        let interval = super::get_day_time_interval(hour,minutes,seconds);
        assert_eq!(String::from("00:01:00"), interval);
    }

    #[test]
    fn test_get_day_time_interval_8() {
        let hour: i64 = 0;
        let minutes: i64 =  0;
        let seconds: f64 = 1.0;
        let interval = super::get_day_time_interval(hour,minutes,seconds);
        assert_eq!(String::from("00:00:01"), interval);
    }

    #[test]
    fn test_get_day_time_interval_9() {
        let hour: i64 = 0;
        let minutes: i64 =  0;
        let seconds: f64 = 1.25;
        let interval = super::get_day_time_interval(hour,minutes,seconds);
        assert_eq!(String::from("00:00:01.25"), interval);
    }

    #[test]
    fn test_get_day_time_interval_10() {
        let hour: i64 = 0;
        let minutes: i64 =  0;
        let seconds: f64 = 0.0;
        let interval = super::get_day_time_interval(hour,minutes,seconds);
        assert_eq!(String::from("00:00:00"), interval);
    }

}
