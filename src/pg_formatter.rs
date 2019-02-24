use pg_interval::Interval;
use std::ops::Neg;

pub struct IntervalNorm {
    pub years: i32,
    pub months: i32,
    pub days: i32,
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
    pub microseconds: i64,
}

impl<'a> From<&'a Interval> for IntervalNorm {
    fn from(val: &Interval) -> IntervalNorm {
        // grab the base values from the interval
        let months = val.months;
        let days = val.days;
        let microseconds = val.microseconds;
        // calc the year and get the remaining months
        let years = (months - (months % 12)) / 12;
        let months = months - years * 12;
        // calc the hours from the microseconds and update
        // the remaining microseconds.
        let hours = (microseconds - (microseconds % 3_600_000_000)) / 3_600_000_000;
        let microseconds = microseconds - hours * 3_600_000_000;
        // calc the minutes from remaining microseconds and
        // update the remaining microseconds.
        let minutes = (microseconds - (microseconds % 60_000_000)) / 60_000_000;
        let microseconds = microseconds - minutes * 60_000_000;
        // calc the seconds and update the remaining microseconds.
        let seconds = (microseconds - (microseconds % 1_000_000)) / 1_000_000;
        let microseconds = microseconds - seconds * 1_000_000;
        IntervalNorm {
            years,
            months,
            days,
            hours,
            minutes,
            seconds,
            microseconds,
        }
    }
}

impl IntervalNorm {
    /// Is all the values in the interval set to 0?
    fn is_zeroed(&self) -> bool {
           self.years == 0
            && self.months == 0
            && self.hours == 0
            && self.minutes == 0
            && self.seconds == 0
            && self.microseconds == 0
    }

    /// Is the years or month value set?
    fn is_year_month_present(&self) -> bool {
        self.years != 0 || self.months != 0
    }

    /// Is the day value set?
    fn is_day_present(&self) -> bool {
        self.days != 0
    }

    /// Is at least one of hours,minutes,seconds,microseconds values
    /// postive. There are no mixed intervals so we can assume that
    /// if one value is postive the rest are at least >= 0
    fn is_time_interval_pos(&self) -> bool {
        self.hours > 0 || self.minutes > 0 || self.seconds > 0 || self.microseconds > 0
    }

    /// Is the hours,minutes, seconds, microseconds values set?
    fn is_time_present(&self) -> bool {
        self.hours != 0 || self.minutes != 0 || self.seconds != 0 || self.microseconds != 0
    }

    /// The sql and postgres standard for the time intervals format
    /// are very similar. This gets the base string that is
    /// shared between them.
    fn get_sql_postgres_time_interval(&self) -> String {
        let mut time_interval = "".to_owned();
        if self.is_time_present() {
            let sign = if !self.is_time_interval_pos() {
                "-".to_owned()
            } else {
                "".to_owned()
            };
            time_interval.push_str(
                &*(sign
                    + &pad_i64(self.hours)
                    + ":"
                    + &pad_i64(self.minutes)
                    + ":"
                    + &pad_i64(self.seconds)),
            );
            if self.microseconds != 0 {
                time_interval.push_str(&*format!(".{:06}", self.microseconds))
            }
        }
        time_interval
    }

    /// Produces a iso 8601 compliant interval string.
    pub fn into_iso_8601(self) -> String {
        if self.is_zeroed() {
            return "PT0S".to_owned();
        }
        let mut year_interval = "P".to_owned();
        let mut day_interval = "".to_owned();
        let mut time_interval;
        if self.is_time_present() {
            time_interval = "T".to_owned();
            if self.hours != 0 {
                time_interval.push_str(&format!("{}H", self.hours));
            }
            if self.minutes != 0 {
                time_interval.push_str(&format!("{}M", self.minutes));
            }
            if self.seconds != 0 {
                time_interval.push_str(&format!("{}S", self.seconds));
            }
            if self.microseconds != 0 {
                let ms = safe_abs_u64(self.microseconds);
                time_interval.push_str(&format!(".{:06}", ms));
            }
        } else {
            time_interval = "".to_owned();
        }
        if self.years != 0 {
            year_interval.push_str(&format!("{}Y", self.years));
        }
        if self.months != 0 {
            year_interval.push_str(&format!("{}M", self.months));
        }
        if self.days != 0 {
            day_interval.push_str(&format!("{}D", self.days));
        }
        year_interval.push_str(&*day_interval);
        year_interval.push_str(&*time_interval);
        year_interval
    }

    /// Produces a postgres compliant interval string.
    pub fn into_postgres(self) -> String {
        if self.is_zeroed() {
            return "00:00:00".to_owned();
        }
        let mut year_interval = "".to_owned();
        let mut day_interval = "".to_owned();
        let time_interval = self.get_sql_postgres_time_interval();
        if self.is_day_present() {
            day_interval = format!("{:#?} days ", self.days)
        }
        if self.is_year_month_present() {
            if self.years != 0 {
                year_interval.push_str(&*format!("{:#?} year ", self.years))
            }
            if self.months != 0 {
                year_interval.push_str(&*format!("{:#?} mons ", self.months));
            }
        }
        year_interval.push_str(&*day_interval);
        year_interval.push_str(&*time_interval);
        year_interval.trim().to_owned()
    }

    /// Produces a compliant sql interval string.
    pub fn into_sql(self) -> String {
        if self.is_zeroed() {
            return "0".to_owned();
        }
        let mut year_interval = "".to_owned();
        let mut day_interval = "".to_owned();
        let mut time_interval = self.get_sql_postgres_time_interval();
        if self.is_time_interval_pos() {
            time_interval = "+".to_owned() + &time_interval
        }
        if self.is_day_present() {
            day_interval = format!("{:#?} ", self.days);
        }
        if self.is_year_month_present() {
            let sign = if self.years < 0 || self.months < 0 {
                "-".to_owned()
            } else {
                "+".to_owned()
            };
            year_interval = format!("{}{}-{} ", sign, self.years, self.months);
        }
        year_interval.push_str(&day_interval);
        year_interval.push_str(&time_interval);
        year_interval.trim().to_owned()
    }
}

/// Safely maps a i64 value to a unsigned number
/// without any overflow issues.
fn safe_abs_u64(mut num: i64) -> u64 {
    let max = i64::max_value();
    let max_min = max.neg();
    if num <= max_min {
        let result = max as u64;
        num += max;
        num *= -1;
        result + num as u64
    } else {
        num.abs() as u64
    }
}

/// Pads a i64 value with a width of 2.
fn pad_i64(val: i64) -> String {
    let num = if val < 0 {
        safe_abs_u64(val)
    } else {
        val as u64
    };
    return format!("{:02}", num);
}
