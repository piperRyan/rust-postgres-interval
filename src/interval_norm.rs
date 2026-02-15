use crate::{Interval, interval_parse::parse_error::ParseError};

pub struct IntervalNorm {
    pub years: i32,
    pub months: i32,
    pub days: i32,
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
    pub microseconds: i64,
}

impl From<&Interval> for IntervalNorm {
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
    pub fn try_into_interval(self) -> Result<Interval, ParseError> {
        let months = self
            .years
            .checked_mul(12)
            .and_then(|years| self.months.checked_add(years));
        let microseconds = self
            .hours
            .checked_mul(60)
            .and_then(|minutes| self.minutes.checked_add(minutes))
            .and_then(|minutes| minutes.checked_mul(60))
            .and_then(|seconds| self.seconds.checked_add(seconds))
            .and_then(|seconds| seconds.checked_mul(1_000_000))
            .and_then(|microseconds| self.microseconds.checked_add(microseconds));
        Ok(Interval {
            months: months.ok_or_else(|| {
                ParseError::from_year_month("Invalid year/month interval overflow detected.")
            })?,
            days: self.days,
            microseconds: microseconds
                .ok_or_else(|| ParseError::from_time("Invalid time interval overflow detected."))?,
        })
    }

    pub fn default() -> IntervalNorm {
        IntervalNorm {
            years: 0,
            months: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            microseconds: 0,
        }
    }
    /// Is all the values in the interval set to 0?
    pub fn is_zeroed(&self) -> bool {
        self.years == 0
            && self.months == 0
            && self.days == 0
            && self.hours == 0
            && self.minutes == 0
            && self.seconds == 0
            && self.microseconds == 0
    }

    /// Is the years or month value set?
    pub fn is_year_month_present(&self) -> bool {
        self.years != 0 || self.months != 0
    }

    /// Is the day value set?
    pub fn is_day_present(&self) -> bool {
        self.days != 0
    }

    /// Is at least one of hours,minutes,seconds,microseconds values
    /// postive. There are no mixed intervals so we can assume that
    /// if one value is postive the rest are at least >= 0
    pub fn is_time_interval_pos(&self) -> bool {
        self.hours > 0 || self.minutes > 0 || self.seconds > 0 || self.microseconds > 0
    }

    /// Is the hours,minutes, seconds, microseconds values set?
    pub fn is_time_present(&self) -> bool {
        self.hours != 0 || self.minutes != 0 || self.seconds != 0 || self.microseconds != 0
    }
}
