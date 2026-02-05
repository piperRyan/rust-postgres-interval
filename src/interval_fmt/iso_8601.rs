use crate::interval_norm::IntervalNorm;

impl IntervalNorm {
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
            if self.seconds != 0 || self.microseconds != 0 {
                let secs_with_micros = if self.seconds != 0 && self.microseconds != 0 {
                    format!(
                        "{}.{:06}",
                        self.seconds,
                        super::safe_abs_u64(self.microseconds)
                    )
                } else if self.microseconds != 0 {
                    format!(".{:06}", super::safe_abs_u64(self.microseconds))
                } else {
                    format!("{}", self.seconds)
                };
                time_interval.push_str(&format!("{}S", secs_with_micros));
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
        year_interval.push_str(&day_interval);
        year_interval.push_str(&time_interval);
        year_interval
    }
}
