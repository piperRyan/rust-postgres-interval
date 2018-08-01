use super::safe_abs_u32;

fn has_year_month_time(years: i32, 
                       months: i32) -> bool {
    years != 0 || months != 0
}

fn has_day_time(days: i32, 
                hours: i64,
                minutes: i64, 
                seconds: f64) -> bool {
    days != 0 || hours != 0 || minutes != 0 || seconds != 0.0
}

fn has_day(days: i32) -> bool {
    days != 0
}


pub fn fmt_to_sql_standard(years: i32,
                       months: i32,
                       days:i32,
                       hours: i64,
                       minutes: i64,
                       seconds: f64) 
                       -> String {
    let has_year_month_time = has_year_month_time(years, months); 
    let has_day_time = has_day_time(days, hours, minutes, seconds); 
    let has_day = has_day(days);
    if !has_year_month_time && !has_day_time {
        "0".to_owned()
    } else if has_year_month_time && !has_day_time {
        let sign; 
        if years < 0 || months < 0 {
            sign = "-"; 
        } else {
            sign = "";
        }
        format!("{}{:01}-{:01}", sign, years.abs(), months.abs())
    } else if !has_year_month_time && has_day_time && has_day {
        let day_sign; 
        if days < 0 {
            day_sign = "-"; 
        } else {
            day_sign = ""; 
        }
        let time_sign; 
        if hours < 0 || minutes < 0 || seconds < 0.0 {
            time_sign = "-"; 
        } else {
            time_sign = "";
        }
        format!("{}{} {}{:01}:{:02}:{:02}",
                 day_sign,
                 safe_abs_u32(days),
                 time_sign,
                 hours.abs(),
                 months.abs(),
                 seconds.abs())
    } else if !has_year_month_time && has_day_time && !has_day {
        let time_sign; 
        if hours < 0 || minutes < 0 || seconds < 0.0 {
            time_sign = "-"; 
        } else {
            time_sign = "";
        }
        format!("{}{:01}:{:02}:{:02}", time_sign, hours.abs(), minutes.abs(), seconds.abs())
    } else {
        let year_sign; 
        if years < 0 || months < 0 {
            year_sign = "-"; 
        } else {
            year_sign = "+"; 
        }
        let time_sign;
        if hours < 0 || minutes < 0 || seconds < 0.0 {
            time_sign = "-"; 
        } else {
            time_sign = "+";
        }
        let day_value; 
        if has_day {
            if days < 0 {
                day_value  = format!("-{}", safe_abs_u32(days)); 
            }  else {
                day_value = format!("+{}", safe_abs_u32(days)); 
            }
        } else {
            day_value = "".to_owned();
        }
        format!("{}{}-{} {} {}{:01}:{:02}:{:02}", 
                year_sign,
                years.abs(),
                months.abs(),
                &*day_value,
                time_sign,
                hours.abs(), 
                minutes.abs(), 
                seconds.abs())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_should_have_year_month() {
        let years = 1; 
        let months = 0; 
        let expected_result = true; 
        let actual_result = super::has_year_month_time(years, months); 
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn it_should_not_have_year_month() {
        let years = 0; 
        let months = 0; 
        let expected_result = false; 
        let actual_result = super::has_year_month_time(years, months);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn it_should_have_day_time() {
        let actual_result = super::has_day_time(0,1,0,1.0); 
        let expected_result = true; 
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn it_should_not_have_day_time() {
        let actual_result = super::has_day_time(0,0,0,0.0); 
        let expected_result = false; 
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn it_should_have_day() {
        let actual_result = super::has_day(1); 
        let expected_result = true; 
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn is_should_format_to_zero() {
        let actual_result = super::fmt_to_sql_standard(0,0,0,0,0,0.0);
        let expected_result = "0".to_owned();
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn is_should_format_to_complete_date() {
        let actual_result = super::fmt_to_sql_standard(6,5,4,3,2,1.0);
        let expected_result = "+6-5 +4 +3:02:01".to_owned();
        println!("{}", actual_result); 
        assert_eq!(actual_result, expected_result);
    }
}
