// Helper function to help derive the year month interval for a iso-8601
// compliant string.
pub fn get_year_month_interval(years: i32, months: i32, days: i32) -> String {
    if years != 0 && months != 0  && days != 0 {
        format!("P{:#?}Y{:#?}M{:#?}D", years, months, days)
    } else if years != 0 && months != 0 && days == 0 {
        format!("P{:#?}Y{:#?}M", years, months)
    } else if years != 0 && months == 0 && days == 0 {
        format!("P{:#?}Y", years)
    } else if years == 0 && months != 0 && days != 0 {
        format!("P{:#?}M{:#?}D", months, days)
    } else if years == 0 && months != 0 && days == 0  {
        format!("P{:#?}M", months)
    } else if years == 0 && months == 0 && days != 0 {
        format!("P{:#?}D", days)
    } else if years != 0 && months == 0 && days != 0 {
        format!("P{:#?}Y{:#?}D", years, days)
    } else {
        format!("P")
    }
}

// Helper function to help derive the day-time interval for a iso-8601
// compliant string.
pub fn get_day_time_interval(hours: i64, minutes: i64, seconds: f64) -> String {
    let has_frac = seconds.fract() == 0.0;
    if hours != 0 && minutes != 0  && seconds != 0.0 {
        if !has_frac {
            format!("T{:#?}H{:#?}M{:#?}S", hours, minutes, seconds)
        } else {
            format!("T{:#?}H{:#?}M{:#?}S", hours, minutes, seconds as i64)
        }
    } else if hours != 0 && minutes != 0 && seconds == 0.0 {
        format!("T{:#?}H{:#?}M", hours, minutes)
    } else if hours != 0 && minutes == 0 && seconds == 0.0 {
        format!("T{:#?}H", hours)
    } else if hours == 0 && minutes != 0 && seconds != 0.0 {
        if !has_frac {
            format!("T{:#?}M{:#?}S", minutes, seconds)
        } else {
            format!("T{:#?}M{:#?}S", minutes, seconds as i64)
        }
    } else if hours == 0 && minutes != 0 && seconds == 0.0  {
        format!("T{:#?}M", minutes)
    } else if hours == 0 && minutes == 0 && seconds != 0.0 {
        if !has_frac {
            format!("T{:#?}S", seconds)
        } else {
            format!("T{:#?}S", seconds as i64)
        }
    } else if hours != 0 && minutes == 0 && seconds != 0.0 {
        if !has_frac {
            format!("T{:#?}H{:#?}S", hours, seconds)
        } else {
            format!("T{:#?}H{:#?}S", hours, seconds as i64)
        }
    } else {
        format!("T0S")
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
        assert_eq!(String::from("P1Y2M21D"), interval);
    }

    #[test]
    fn test_get_year_month_interval_2() {
        let year: i32 =0;
        let months: i32 =  2;
        let days: i32 = 21;
        let interval = super::get_year_month_interval(year, months, days);
        assert_eq!(String::from("P2M21D"), interval);
    }

    #[test]
    fn test_get_year_month_interval_3() {
        let year: i32 =0;
        let months: i32 =  0;
        let days: i32 = 21;
        let interval = super::get_year_month_interval(year, months, days);
        assert_eq!(String::from("P21D"), interval);
    }

    #[test]
    fn test_get_year_month_interval_4() {
        let year: i32 =0;
        let months: i32 =  0;
        let days: i32 = 0;
        let interval = super::get_year_month_interval(year, months, days);
        assert_eq!(String::from("P"), interval);
    }

    #[test]
    fn test_get_year_month_interval_5() {
        let year: i32 =1;
        let months: i32 =  12;
        let days: i32 = 0;
        let interval = super::get_year_month_interval(year, months, days);
        assert_eq!(String::from("P1Y12M"), interval);
    }

    #[test]
    fn test_get_year_month_interval_6() {
        let year: i32 =1;
        let months: i32 =  0;
        let days: i32 = 21;
        let interval = super::get_year_month_interval(year, months, days);
        assert_eq!(String::from("P1Y21D"), interval);
    }

    #[test]
    fn test_get_year_month_interval_7() {
        let year: i32 =1;
        let months: i32 =  0;
        let days: i32 = 0;
        let interval = super::get_year_month_interval(year, months, days);
        assert_eq!(String::from("P1Y"), interval);
    }

    #[test]
    fn test_get_year_month_interval_8() {
        let year: i32 =0;
        let months: i32 =  1;
        let days: i32 = 0;
        let interval = super::get_year_month_interval(year, months, days);
        assert_eq!(String::from("P1M"), interval);
    }

    #[test]
    fn test_get_day_time_interval_1() {
        let hour: i64 = 1;
        let minutes: i64 =  1;
        let seconds: f64 = 1.25;
        let interval = super::get_day_time_interval(hour,minutes,seconds);
        assert_eq!(String::from("T1H1M1.25S"), interval);
    }

    #[test]
    fn test_get_day_time_interval_2() {
        let hour: i64 = 1;
        let minutes: i64 =  1;
        let seconds: f64 = 1.0;
        let interval = super::get_day_time_interval(hour,minutes,seconds);
        assert_eq!(String::from("T1H1M1S"), interval);
    }

    #[test]
    fn test_get_day_time_interval_3() {
        let hour: i64 = 1;
        let minutes: i64 =  1;
        let seconds: f64 = 0.0;
        let interval = super::get_day_time_interval(hour,minutes,seconds);
        assert_eq!(String::from("T1H1M"), interval);
    }

    #[test]
    fn test_get_day_time_interval_4() {
        let hour: i64 = 1;
        let minutes: i64 =  0;
        let seconds: f64 = 1.24;
        let interval = super::get_day_time_interval(hour,minutes,seconds);
        assert_eq!(String::from("T1H1.24S"), interval);
    }

    #[test]
    fn test_get_day_time_interval_5() {
        let hour: i64 = 0;
        let minutes: i64 =  1;
        let seconds: f64 = 1.24;
        let interval = super::get_day_time_interval(hour,minutes,seconds);
        assert_eq!(String::from("T1M1.24S"), interval);
    }

    #[test]
    fn test_get_day_time_interval_6() {
        let hour: i64 = 1;
        let minutes: i64 =  0;
        let seconds: f64 = 0.0;
        let interval = super::get_day_time_interval(hour,minutes,seconds);
        assert_eq!(String::from("T1H"), interval);
    }

    #[test]
    fn test_get_day_time_interval_7() {
        let hour: i64 = 0;
        let minutes: i64 =  1;
        let seconds: f64 = 0.0;
        let interval = super::get_day_time_interval(hour,minutes,seconds);
        assert_eq!(String::from("T1M"), interval);
    }

    #[test]
    fn test_get_day_time_interval_8() {
        let hour: i64 = 0;
        let minutes: i64 =  0;
        let seconds: f64 = 1.0;
        let interval = super::get_day_time_interval(hour,minutes,seconds);
        assert_eq!(String::from("T1S"), interval);
    }

    #[test]
    fn test_get_day_time_interval_9() {
        let hour: i64 = 0;
        let minutes: i64 =  0;
        let seconds: f64 = 1.25;
        let interval = super::get_day_time_interval(hour,minutes,seconds);
        assert_eq!(String::from("T1.25S"), interval);
    }

    #[test]
    fn test_get_day_time_interval_10() {
        let hour: i64 = 0;
        let minutes: i64 =  0;
        let seconds: f64 = 0.0;
        let interval = super::get_day_time_interval(hour,minutes,seconds);
        assert_eq!(String::from("T0S"), interval);
    }

    #[test]
    fn test_get_day_time_interval_11() {
        let hour: i64 = 0;
        let minutes: i64 =  1;
        let seconds: f64 = 1.0;
        let interval = super::get_day_time_interval(hour,minutes,seconds);
        assert_eq!(String::from("T1M1S"), interval);
    }

    #[test]
    fn test_get_day_time_interval_12() {
        let hour: i64 = 1;
        let minutes: i64 =  0;
        let seconds: f64 = 1.0;
        let interval = super::get_day_time_interval(hour,minutes,seconds);
        assert_eq!(String::from("T1H1S"), interval);
    }



}
