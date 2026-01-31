use crate::Interval;
use std::ops;

impl Interval {
    /// Checked interval addition. Computes `Interval + Interval` and `None` if there
    /// was an overflow.
    pub fn checked_add(self, other_interval: Interval) -> Option<Interval> {
        Some(Interval {
            months: self.months.checked_add(other_interval.months)?,
            days: self.days.checked_add(other_interval.days)?,
            microseconds: self.microseconds.checked_add(other_interval.microseconds)?,
        })
    }

    /// Shortcut method to add day time part to the interval. Any units smaller than a microsecond
    /// will be truncated.
    pub fn add_day_time(self, days: i32, hours: i64, minutes: i64, seconds: f64) -> Interval {
        let hours_as_micro: i64 = hours * 3_600_000_000;
        let minutes_as_micro: i64 = minutes * 60_000_000;
        let seconds_as_micro: i64 = (seconds * 1_000_000.0).floor() as i64;
        let additional_micro: i64 = hours_as_micro + minutes_as_micro + seconds_as_micro;
        Interval {
            months: self.months,
            days: self.days + days,
            microseconds: self.microseconds + additional_micro,
        }
    }

    /// Checked day time interval addition. Computes the interval and will return `None` if a
    /// overflow has occured. Any units smaller than a microsecond will be truncated.
    pub fn checked_add_day_time(
        self,
        days: i32,
        hours: i64,
        minutes: i64,
        seconds: f64,
    ) -> Option<Interval> {
        let hours_as_micro: i64 = hours.checked_mul(3_600_000_000)?;
        let minutes_as_micro: i64 = minutes.checked_mul(60_000_000)?;
        let seconds_as_micro: i64 = (seconds * 1_000_000.0).floor() as i64;
        let additional_micro: i64 = hours_as_micro
            .checked_add(minutes_as_micro)?
            .checked_add(seconds_as_micro)?;
        Some(Interval {
            months: self.months,
            days: self.days.checked_add(days)?,
            microseconds: self.microseconds.checked_add(additional_micro)?,
        })
    }

    /// Adds a year month interval.
    pub fn add_year_month(self, year: i32, months: i32) -> Interval {
        let years_as_months = year * 12;
        Interval {
            months: self.months + years_as_months + months,
            days: self.days,
            microseconds: self.microseconds,
        }
    }

    /// Checked year month addition. Computes the interval and will return `None` if a
    /// overflow has occured.
    pub fn checked_add_year_month(self, year: i32, months: i32) -> Option<Interval> {
        let years_as_months = year.checked_mul(12)?;
        let additional_months = years_as_months.checked_add(months)?;
        Some(Interval {
            months: self.months.checked_add(additional_months)?,
            days: self.days,
            microseconds: self.microseconds,
        })
    }
}

impl ops::Add for Interval {
    type Output = Interval;
    fn add(self, other_interval: Interval) -> Interval {
        Interval {
            months: self.months + other_interval.months,
            days: self.days + other_interval.days,
            microseconds: self.microseconds + other_interval.microseconds,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_checked_add() {
        let interval = Interval::new(13, 0, 0);
        let interval_add = Interval::new(2, 1, 12);
        let result = interval.checked_add(interval_add);
        assert_eq!(result, Some(Interval::new(15, 1, 12)));
    }

    #[test]
    fn test_checked_add_2() {
        let interval = Interval::new(13, 0, 0);
        let interval_add = Interval::new(i32::MAX, 1, 12);
        let result = interval.checked_add(interval_add);
        assert_eq!(result, None);
    }

    #[test]
    fn test_checked_add_day_time() {
        let interval = Interval::new(13, 0, 0);
        let result = interval.checked_add_day_time(2, 0, 0, 2.123456789);
        assert_eq!(result, Some(Interval::new(13, 2, 2123456)));
    }

    #[test]
    fn test_checked_add_day_time_2() {
        let interval = Interval::new(13, i32::MAX, 0);
        let result = interval.checked_add_day_time(200, 0, 0, 2.123456789);
        assert_eq!(result, None);
    }

    #[test]
    fn test_add_year_month() {
        let interval = Interval::new(13, 0, 0);
        let result = interval.add_year_month(1, 1);
        assert_eq!(result, Interval::new(26, 0, 0));
    }

    #[test]
    fn checked_add_year_month_1() {
        let interval = Interval::new(15, 0, 0);
        let result = interval.checked_add_year_month(1, 1);
        assert_eq!(result, Some(Interval::new(28, 0, 0)));
    }

    #[test]
    fn checked_add_year_month_2() {
        let interval = Interval::new(15, 0, 0);
        let result = interval.checked_add_year_month(i32::MAX, 32);
        assert_eq!(result, None);
    }

    #[test]
    fn test_add() {
        let interval = Interval::new(0, 0, 0);
        let other_interval = Interval::new(10, 0, 0);
        let result = interval + other_interval;
        assert_eq!(result, Interval::new(10, 0, 0));
    }

    #[test]
    fn test_add_day_time() {
        let interval = Interval::new(13, 0, 0);
        let result = interval.add_day_time(2, 0, 0, 2.123456789);
        assert_eq!(result, Interval::new(13, 2, 2123456));
    }
}
