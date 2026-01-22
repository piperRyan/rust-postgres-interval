use crate::interval_norm::IntervalNorm;

impl IntervalNorm {
    pub fn into_sql(self) -> String {
        if self.is_zeroed() {
            "0".to_owned()
        } else if !self.is_time_present() && !self.is_day_present() {
            get_year_month(self.months, self.years, true)
        } else if !self.is_time_present() && !self.is_year_month_present() {
            format!("{} 0:00:00", self.days)
        } else if !self.is_year_month_present() && !self.is_day_present() {
            get_time_interval(
                self.hours,
                self.minutes,
                self.seconds,
                self.microseconds,
                self.is_time_interval_pos(),
                true,
            )
        } else {
            let year_month = get_year_month(self.months, self.years, false);
            let time_interval = get_time_interval(
                self.hours,
                self.minutes,
                self.seconds,
                self.microseconds,
                self.is_time_interval_pos(),
                false,
            );
            format!("{} {:+} {}", year_month, self.days, time_interval)
        }
    }
}

fn get_year_month(mons: i32, years: i32, is_only_year_month: bool) -> String {
    let months = super::safe_abs_u32(mons);
    if years == 0 || is_only_year_month {
        format!("{}-{}", years, months)
    } else {
        format!("{:+}-{}", years, months)
    }
}

fn get_time_interval(
    hours: i64,
    mins: i64,
    secs: i64,
    micros: i64,
    is_time_interval_pos: bool,
    is_only_time: bool,
) -> String {
    let mut interval = "".to_owned();
    if is_time_interval_pos && is_only_time {
        interval.push_str(&format!("{:02}:{:02}:{:02}", hours, mins, secs));
    } else {
        let sign = if hours < 0 { "-" } else { "+" };
        let hours_abs = super::safe_abs_u64(hours);
        let minutes = super::safe_abs_u64(mins);
        let seconds = super::safe_abs_u64(secs);
        interval.push_str(&format!(
            "{}{:02}:{:02}:{:02}",
            sign, hours_abs, minutes, seconds
        ));
    }
    if micros != 0 {
        let microseconds = format!(".{:06}", super::safe_abs_u64(micros));
        interval.push_str(&microseconds);
    }
    interval
}
