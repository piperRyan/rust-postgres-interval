use interval_norm::IntervalNorm;

impl IntervalNorm {
    /// Produces a postgres compliant interval string.
    pub fn into_postgres(self) -> String {
        if self.is_zeroed() {
            return "00:00:00".to_owned();
        }
        let mut year_interval = "".to_owned();
        let mut day_interval = "".to_owned();
        let time_interval = self.get_postgres_time_interval();
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

    fn get_postgres_time_interval(&self) -> String {
        let mut time_interval = "".to_owned();
        if self.is_time_present() {
            let sign = if !self.is_time_interval_pos() {
                "-".to_owned()
            } else {
                "".to_owned()
            };
            let hours = super::pad_i64(self.hours);
            time_interval.push_str(
                &*(sign
                    + &hours
                    + ":"
                    + &super::pad_i64(self.minutes)
                    + ":"
                    + &super::pad_i64(self.seconds)),
            );
            if self.microseconds != 0 {
                time_interval.push_str(&*format!(".{:06}", super::safe_abs_u64(self.microseconds)))
            }
        }
        time_interval
    }
}
