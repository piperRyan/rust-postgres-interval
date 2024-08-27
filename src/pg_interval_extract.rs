use crate::pg_interval::Interval;
/// Extracts units from an interval.
/// Complying with postgres interval format and date_part function.
impl Interval {
    const NUMBER_OF_MICROSECONDS_IN_MILLISECOND: i64 = 1_000;
    const NUMBER_OF_MICROSECONDS_IN_SECOND: i64 = 1_000 * Self::NUMBER_OF_MICROSECONDS_IN_MILLISECOND;
    const NUMBER_OF_MICROSECONDS_IN_MINUTE: i64 = 60 * Self::NUMBER_OF_MICROSECONDS_IN_SECOND;
    const NUMBER_OF_MICROSECONDS_IN_HOUR: i64 = 60 * Self::NUMBER_OF_MICROSECONDS_IN_MINUTE;
    pub fn years(&self) -> i32 {
        self.months / 12
    }

    pub fn months(&self) -> i32 {
        self.months % 12
    }

    pub fn days(&self) -> i32 {
        self.days.clone()
    }

    pub fn hours(&self) -> i64 {
        self.microseconds / Self::NUMBER_OF_MICROSECONDS_IN_HOUR
    }

    pub fn minutes(&self) -> i64 {
        (self.microseconds % Self::NUMBER_OF_MICROSECONDS_IN_HOUR) / Self::NUMBER_OF_MICROSECONDS_IN_MINUTE
    }

    pub fn seconds(&self) -> f64 {
        (self.microseconds % Self::NUMBER_OF_MICROSECONDS_IN_MINUTE) as f64 / Self::NUMBER_OF_MICROSECONDS_IN_SECOND as f64
    }

    pub fn milliseconds(&self) -> f64 {
        ((self.microseconds) % Self::NUMBER_OF_MICROSECONDS_IN_MINUTE) as f64 / 1_000.0
    }

    pub fn microseconds(&self) -> i64 {
        self.microseconds % Self::NUMBER_OF_MICROSECONDS_IN_MINUTE
    }

    pub fn epoch(&self) -> f64 {
        // To comply with postgres we need to first calculate number of seconds in years.
        // each is 365.25 days
        let years_seconds = self.years() as f64 * 31557600.0;
        // Calculate number of seconds in months. Each month is 30 days
        let months_seconds = self.months() as f64 * 2592000.0;
        years_seconds + months_seconds + self.days as f64 * 86400.0 + self.microseconds as f64 / Self::NUMBER_OF_MICROSECONDS_IN_SECOND as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_years() {
        let interval = Interval::from_postgres("2000 years 6 months 15 days 10 hours 30 minutes 20.123456 seconds").unwrap();
        assert_eq!(interval.years(), 2000);
    }

    #[test]
    fn test_months() {
        let interval = Interval::from_postgres("2000 years 6 months 15 days 10 hours 30 minutes 20.123456 seconds").unwrap();
        assert_eq!(interval.months(), 6);
    }

    #[test]
    fn test_days() {
        let interval = Interval::from_postgres("2000 years 6 months 15 days 10 hours 30 minutes 20.123456 seconds").unwrap();
        assert_eq!(interval.days(), 15);
    }

    #[test]
    fn test_hours() {
        let interval = Interval::from_postgres("2000 years 6 months 15 days 10 hours 30 minutes 20.123456 seconds").unwrap();
        assert_eq!(interval.hours(), 10);
    }

    #[test]
    fn test_minutes() {
        let interval = Interval::from_postgres("2000 years 6 months 15 days 10 hours 30 minutes 20.123456 seconds").unwrap();
        assert_eq!(interval.minutes(), 30);
    }

    #[test]
    fn test_seconds() {
        let interval = Interval::from_postgres("2000 years 6 months 15 days 10 hours 30 minutes 20.123456 seconds").unwrap();
        assert_eq!(interval.seconds(), 20.123456);
    }

    #[test]
    fn test_milliseconds() {
        let interval = Interval::from_postgres("2000 years 6 months 15 days 10 hours 30 minutes 20.123456 seconds").unwrap();
        assert_eq!(interval.milliseconds(), 20123.456);
    }

    #[test]
    fn test_microseconds() {
        let interval = Interval::from_postgres("2000 years 6 months 15 days 10 hours 30 minutes 20.123456 seconds").unwrap();
        assert_eq!(interval.microseconds(), 20123456);
        let interval = Interval::from_postgres("2023 years 3 months 5 days 22 hours 18 minutes 41.228289 seconds").unwrap();
        assert_eq!(interval.microseconds(), 41228289);
    }

    #[test]
    fn epoch() {
        let interval = Interval::from_postgres("2000 years 6 months 15 days 10 hours 30 minutes 20.123456 seconds").unwrap();
        assert_eq!(interval.epoch(), 63132085820.123456);
    }
}