use std::error::Error;
use std::fmt;
use crate::Interval;
use chrono::{Days, Duration, Months, NaiveDateTime};

#[derive(Debug)]
pub enum OperationError {
    MonthsOverflow,
    DaysOverflow,
    SecondsOverflow,
    MicroSecondsOverflow,
}

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OperationError::MonthsOverflow => write!(f, "Overflow when adding months"),
            OperationError::DaysOverflow => write!(f, "Overflow when adding days"),
            OperationError::SecondsOverflow => write!(f, "Overflow when adding seconds"),
            OperationError::MicroSecondsOverflow => write!(f, "Overflow when adding microseconds"),
        }
    }
}

impl Error for OperationError {}

impl Interval {
    /// Tries to convert from the `Duration` type to a `Interval`. Will
    /// return `None` on a overflow. This is a lossy conversion in that
    /// any units smaller than a microsecond will be lost.
    pub fn add_to_datetime(&self, mut d: NaiveDateTime) -> Result<NaiveDateTime, OperationError> {
        let abs_months = Months::new(self.months.abs() as u32);
        d = if self.months > 0 {
            d.checked_add_months(abs_months)
        } else {
            d.checked_sub_months(abs_months)
        }.ok_or(OperationError::MonthsOverflow)?;

        let abs_days = Days::new(self.days.abs() as u64);
        d = if self.days > 0 {
            d.checked_add_days(abs_days)
        } else {
            d.checked_sub_days(abs_days)
        }.ok_or(OperationError::DaysOverflow)?;

        d = d.checked_add_signed(Duration::microseconds(self.microseconds)).unwrap();
        Ok(d)
    }
}