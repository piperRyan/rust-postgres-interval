use crate::Interval;
use std::ops;

impl Interval {
    /// Checked interval subtraction. Computes `Interval - Interval` and `None` if there
    /// was an underflow.
    pub fn checked_sub(self, other_interval: Interval) -> Option<Interval> {
        Some(Interval {
            months: self.months.checked_sub(other_interval.months)?,
            days: self.days.checked_sub(other_interval.days)?,
            microseconds: self.microseconds.checked_sub(other_interval.microseconds)?,
        })
    }

    /// Shortcut method to subtract day time part to the interval. Any units smaller than
    /// a microsecond will be truncated.
    pub fn sub_day_time(self, days: i32, hours: i64, minutes: i64, seconds: f64) -> Interval {
        let hours_as_micro: i64 = hours * 3_600_000_000;
        let minutes_as_micro: i64 = minutes * 60_000_000;
        let seconds_as_micro: i64 = (seconds * 1_000_000.0).floor() as i64;
        let additional_micro: i64 = hours_as_micro + minutes_as_micro + seconds_as_micro;
        Interval {
            months: self.months,
            days: self.days - days,
            microseconds: self.microseconds - additional_micro,
        }
    }

    /// Checked day time subtraction. Computes the interval and will return `None` if a
    /// overflow/underflow has occured. Any units smaller than a microsecond will be truncated.
    pub fn checked_sub_day_time(
        self,
        days: i32,
        hours: i64,
        minutes: i64,
        seconds: f64,
    ) -> Option<Interval> {
        let hours_as_micro: i64 = hours.checked_mul(3_600_000_000)?;
        let minutes_as_micro: i64 = minutes.checked_mul(60_000_000)?;
        let seconds_as_micro: i64 = (seconds * 1_000_000.0).floor() as i64;
        let subtracted_micro: i64 = hours_as_micro
            .checked_add(minutes_as_micro)?
            .checked_add(seconds_as_micro)?;
        Some(Interval {
            months: self.months,
            days: self.days.checked_sub(days)?,
            microseconds: self.microseconds.checked_sub(subtracted_micro)?,
        })
    }

    /// Subtracts a year month interval.
    pub fn sub_year_month(self, year: i32, months: i32) -> Interval {
        let years_as_months = year * 12;
        Interval {
            months: self.months - years_as_months - months,
            days: self.days,
            microseconds: self.microseconds,
        }
    }

    /// Checked year month subtraction. Computes the interval and will return `None` if a
    /// overflow has occured.
    pub fn checked_sub_year_month(self, year: i32, months: i32) -> Option<Interval> {
        let years_as_months = year.checked_mul(12)?;
        Some(Interval {
            months: self
                .months
                .checked_sub(years_as_months)?
                .checked_sub(months)?,
            days: self.days,
            microseconds: self.microseconds,
        })
    }
}

impl ops::Sub for Interval {
    type Output = Interval;
    fn sub(self, other_interval: Interval) -> Interval {
        Interval {
            months: self.months - other_interval.months,
            days: self.days - other_interval.days,
            microseconds: self.microseconds - other_interval.microseconds,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_day_time() {
        let interval = Interval::new(13, 0, 0);
        let result = interval.sub_day_time(2, 0, 0, 2.12);
        assert_eq!(result, Interval::new(13, -2, -2120000));
    }

    #[test]
    fn test_checked_sub_day_time() {
        let interval = Interval::new(13, 0, 0);
        let result = interval.checked_sub_day_time(2, 0, 0, 2.12);
        assert_eq!(result, Some(Interval::new(13, -2, -2120000)));
    }

    #[test]
    fn test_checked_sub_day_time_2() {
        let interval = Interval::new(13, i32::min_value(), 0);
        println!("{:?}", interval.days);
        let result = interval.checked_sub_day_time(100, 0, 0, 2.12);
        println!("{:?}", result);
        assert_eq!(result, None);
    }

    #[test]
    fn test_sub_year_month() {
        let interval = Interval::new(13, 0, 0);
        let result = interval.sub_year_month(1, 1);
        assert_eq!(result, Interval::new(0, 0, 0));
    }

    #[test]
    fn test_checked_sub_year_month_1() {
        let interval = Interval::new(13, 0, 0);
        let result = interval.checked_sub_year_month(1, 1);
        assert_eq!(result, Some(Interval::new(0, 0, 0)));
    }

    #[test]
    fn test_checked_sub_year_month_2() {
        let interval = Interval::new(-20, 0, 0);
        let result = interval.checked_sub_year_month(i32::min_value(), 1);
        assert_eq!(result, None);
    }

    #[test]
    fn test_sub() {
        let interval = Interval::new(30, 0, 0);
        let other_interval = Interval::new(20, 0, 0);
        let result = interval - other_interval;
        assert_eq!(result, Interval::new(10, 0, 0));
    }

    #[test]
    fn test_checked_sub() {
        let interval = Interval::new(13, 0, 0);
        let interval_sub = Interval::new(2, 0, 0);
        let result = interval.checked_sub(interval_sub);
        assert_eq!(result, Some(Interval::new(11, 0, 0)));
    }

    #[test]
    fn test_checked_sub_2() {
        let interval = Interval::new(-10, 0, 0);
        let interval_sub = Interval::new(i32::max_value(), 0, 0);
        let result = interval.checked_sub(interval_sub);
        assert_eq!(result, None);
    }

}
