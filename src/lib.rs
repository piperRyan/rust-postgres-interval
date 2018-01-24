pub struct Interval {
  months: i32,
  days: i32,
  microseconds: i64
}

impl Interval {
    /// Create a new instance of interval from the months, days, and microseconds.
    pub fn new(months: i32, days: i32, microseconds: i64) -> Interval {
        // the interval must be either all postive or all negative values.
        debug_assert!(
            (months >= 0 && days >= 0 && microseconds >= 0) ||
            (months <= 0 && days <= 0 && microseconds <= 0)
        );
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
        let mut year_months_interval = get_year_month_interval(years, months, days);
        if self.microseconds != 0  || year_months_interval.as_str() == "P" {
            let (remaining_microseconds, hours) = get_hours(self.microseconds);
            let (remaining_microseconds, minutes) = get_minutes(remaining_microseconds);
            let seconds = get_seconds(remaining_microseconds);
            let day_time_interval = get_day_time_interval(hours, minutes, seconds);
            year_months_interval.push_str(day_time_interval.as_str());
            year_months_interval
        } else {
            year_months_interval
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

// Helper function to help derive the year month interval for a iso-8601
// compliant string.
fn get_year_month_interval(years: i32, months: i32, days: i32) -> String {
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
fn get_day_time_interval(hours: i64, minutes: i64, seconds: f64) -> String {
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
        format!("T{:#?}S", seconds)
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
