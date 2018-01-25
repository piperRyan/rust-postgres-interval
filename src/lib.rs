mod iso_8601;

pub struct Interval {
  months: i32,
  days: i32,
  microseconds: i64
}

impl Interval {
    /// Create a new instance of interval from the months, days, and microseconds.
    pub fn new(months: i32, days: i32, microseconds: i64) -> Interval {
        Interval {
            months: months,
            days: days,
            microseconds: microseconds
        }
    }

    /// Get the amount of months in the interval.
    pub fn months(&self) -> i32 {
        self.months
    }

    /// Get the amount of days in the interval.
    pub fn days(&self) -> i32 {
        self.days
    }

    /// Get the amount of microseconds in the interval
    pub fn microseconds(&self) -> i64 {
        self.microseconds
    }

    /// Output the interval as iso 8601 compliant string.
    pub fn to_iso_8601(&self) -> String {
        let (years, months) = get_years_months(self.months);
        let days = self.days;
        let mut year_months_interval = iso_8601::get_year_month_interval(years, months, days);
        if self.microseconds != 0  || year_months_interval.as_str() == "P" {
            let (remaining_microseconds, hours) = get_hours(self.microseconds);
            let (remaining_microseconds, minutes) = get_minutes(remaining_microseconds);
            let seconds = get_seconds(remaining_microseconds);
            let day_time_interval = iso_8601::get_day_time_interval(hours, minutes, seconds);
            year_months_interval.push_str(day_time_interval.as_str());
            year_months_interval
        } else {
            year_months_interval
        }
    }

    pub fn to_postgres(&self) -> String {
        let (years, months) = get_years_months(self.months);
        let days = self.days;
        let year_months_interval = postgres::get_year_month_interval(years, months, days);
        let (remaining_microseconds, hours) = get_hours(self.microseconds);
        let (remaining_microseconds, minutes) = get_minutes(remaining_microseconds);
        let seconds = get_seconds(remaining_microseconds);
        if self.microseconds != 0 && year_months_interval.is_some() {
                let mut ym_interval = year_months_interval.unwrap();
                let day_time_interval = postgres::get_day_time_interval(hours, minutes, seconds);
                ym_interval = ym_interval + " " + &*day_time_interval;
                ym_interval
        } else if year_months_interval.is_some() && self.microseconds == 0 {
            year_months_interval.unwrap()
        } else {
            postgres::get_day_time_interval(hours, minutes, seconds)
        }
    }
}

// Helper function to derive the amount of years are found in the interval.
fn get_years_months(months: i32) -> (i32, i32) {
    let years = (months - (months % 12))/12;
    let remaining_months = months - years * 12;
    (years, remaining_months)
}

// Helper function to derive the amount of hours are found in the interval.
fn get_hours(current_microseconds: i64) -> (i64, i64) {
    let hours = (current_microseconds - (current_microseconds % 3600000000))/ 3600000000;
    let remaining_microseconds = current_microseconds - hours * 3600000000;
    (remaining_microseconds, hours)
}

// Helper function to derive the amount of minutes are found in the interval.
fn get_minutes(current_microseconds: i64) -> (i64, i64) {
    let minutes = (current_microseconds - (current_microseconds % 60000000)) / 60000000;
    let remaining_microseconds = current_microseconds - minutes * 60000000;
    (remaining_microseconds, minutes)
}

// Helper function to derive the amount of seconds are found in the interval.
fn get_seconds(current_microseconds: i64) -> f64 {
  current_microseconds as f64 / 1000000 as f64
}


#[cfg(test)]
mod tests {
    use super::Interval;
    #[test]
    fn test_get_hours() {
        let (remaining_micro, hours) = super::get_hours(3600000000);
        assert_eq!(remaining_micro, 0);
        assert_eq!(hours, 1);
        let (remaining_micro, hours) = super::get_hours(4320000000);
        assert_eq!(remaining_micro, 720000000);
        assert_eq!(hours, 1);
    }

    #[test]
    fn test_get_neg_hours() {
        let (remaining_micro, hours) = super::get_hours(-3600000000);
        assert_eq!(remaining_micro, 0);
        assert_eq!(hours, -1);
        let (remaining_micro, hours) = super::get_hours(-4320000000);
        assert_eq!(remaining_micro, -720000000);
        assert_eq!(hours, -1);
    }

    #[test]
    fn test_get_minutes() {
        let (remaining_micro, minutes) = super::get_minutes(60000000);
        assert_eq!(remaining_micro, 0);
        assert_eq!(minutes, 1);
        let (remaining_micro, minutes) = super::get_minutes(75000000);
        assert_eq!(remaining_micro, 15000000);
        assert_eq!(minutes, 1);
    }

    #[test]
    fn test_get_neg_minutes() {
        let (remaining_micro, minutes) = super::get_minutes(-60000000);
        assert_eq!(remaining_micro, 0);
        assert_eq!(minutes, -1);
        let (remaining_micro, minutes) = super::get_minutes(-75000000);
        assert_eq!(remaining_micro, -15000000);
        assert_eq!(minutes, -1);
    }

    #[test]
    fn get_years_months_1() {
        let months = 12;
        let (years, months) = super::get_years_months(months);
        assert_eq!(months, 0);
        assert_eq!(years, 1);
        let months = 14;
        let (years, months) = super::get_years_months(months);
        assert_eq!(years, 1);
        assert_eq!(months, 2);
    }

    #[test]
    fn get_years_months_neg_2() {
        let months = -12;
        let (years, months) = super::get_years_months(months);
        assert_eq!(months, 0);
        assert_eq!(years, -1);
        let months = -14;
        let (years, months) = super::get_years_months(months);
        assert_eq!(years, -1);
        assert_eq!(months, -2);
    }



    #[test]
    fn test_get_seconds() {
        let seconds = super::get_seconds(1000000);
        assert_eq!(seconds, 1.0);
        let seconds = super::get_seconds(1250000);
        assert_eq!(seconds, 1.25);
    }

    #[test]
    fn test_get_neg_seconds() {
        let seconds = super::get_seconds(-1000000);
        assert_eq!(seconds, -1.0);
        let seconds = super::get_seconds(-1250000);
        assert_eq!(seconds, -1.25);
    }

    #[test]
    fn test_get_hours_minutes() {
        let (remaining_micro, hours) = super::get_hours(4320000000);
        assert_eq!(remaining_micro, 720000000);
        assert_eq!(hours, 1);
        let (remaining_micro, minutes) = super::get_minutes(remaining_micro);
        assert_eq!(remaining_micro, 0);
        assert_eq!(minutes, 12);
    }

    #[test]
    fn test_get_neg_hours_minutes() {
        let (remaining_micro, hours) = super::get_hours(-4320000000);
        assert_eq!(remaining_micro, -720000000);
        assert_eq!(hours, -1);
        let (remaining_micro, minutes) = super::get_minutes(remaining_micro);
        assert_eq!(remaining_micro, 0);
        assert_eq!(minutes, -12);
    }

    #[test]
    fn test_get_hours_minutes_seconds() {
        let (remaining_micro, hours) = super::get_hours(4509000000);
        assert_eq!(remaining_micro, 909000000);
        assert_eq!(hours, 1);
        let (remaining_micro, minutes) = super::get_minutes(remaining_micro);
        assert_eq!(remaining_micro, 9000000);
        assert_eq!(minutes, 15);
        let seconds : f64 = super::get_seconds(remaining_micro);
        assert_eq!(seconds, 9.0);
    }

    #[test]
    fn test_get_neg_hours_minutes_seconds() {
        let (remaining_micro, hours) = super::get_hours(-4509000000);
        assert_eq!(remaining_micro, -909000000);
        assert_eq!(hours, -1);
        let (remaining_micro, minutes) = super::get_minutes(remaining_micro);
        assert_eq!(remaining_micro, -9000000);
        assert_eq!(minutes, -15);
        let seconds : f64 = super::get_seconds(remaining_micro);
        assert_eq!(seconds, -9.0);
    }

    #[test]
    fn test_new_interval_pos() {
        let interval = Interval::new(1,1,30);
        assert_eq!(interval.months(), 1);
        assert_eq!(interval.days(), 1);
        assert_eq!(interval.microseconds(), 30);
    }

    #[test]
    fn test_new_interval_neg() {
        let interval = Interval::new(-1,-1,-30);
        assert_eq!(interval.months(), -1);
        assert_eq!(interval.days(), -1);
        assert_eq!(interval.microseconds(), -30);
    }
}
