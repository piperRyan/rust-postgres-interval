use crate::interval_norm::IntervalNorm;

fn get_year_suffix(value: i32) -> &'static str {
    if value.abs() == 1 {
        "year"
    } else {
        "years"
    }
}

fn get_mon_suffix(value: i32) -> &'static str {
    if value.abs() == 1 {
        "mon"
    } else {
        "mons"
    }
}

fn get_day_suffix(value: i32) -> &'static str {
    if value.abs() == 1 {
        "day"
    } else {
        "days"
    }
}

fn get_hour_suffix(value: i64) -> &'static str {
    if value.abs() == 1 {
        "hour"
    } else {
        "hours"
    }
}

fn get_min_suffix(value: i64) -> &'static str {
    if value.abs() == 1 {
        "min"
    } else {
        "mins"
    }
}

fn get_sec_suffix(value: i64) -> &'static str {
    if value.abs() == 1 {
        "sec"
    } else {
        "secs"
    }
}

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
            day_interval = format!("{} {} ", self.days, get_day_suffix(self.days))
        }
        if self.is_year_month_present() {
            if self.years != 0 {
                year_interval.push_str(&*format!("{} {} ", self.years, get_year_suffix(self.years)))
            }
            if self.months != 0 {
                year_interval.push_str(&*format!(
                    "{} {} ",
                    self.months,
                    get_mon_suffix(self.months)
                ));
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

    /// Produces a postgres_verbose compliant interval string.
    pub fn into_postgres_verbose(self) -> String {
        let is_negative = !self.is_time_interval_pos()
            && (self.years < 0
                || self.months < 0
                || self.days < 0
                || self.hours < 0
                || self.minutes < 0
                || self.seconds < 0
                || self.microseconds < 0);

        let mut parts = Vec::new();

        if self.years != 0 {
            let abs_years = if self.years < 0 {
                -self.years
            } else {
                self.years
            };
            parts.push(format!("{} {}", abs_years, get_year_suffix(self.years)));
        }

        if self.months != 0 {
            let abs_months = if self.months < 0 {
                -self.months
            } else {
                self.months
            };
            parts.push(format!("{} {}", abs_months, get_mon_suffix(self.months)));
        }

        if self.days != 0 {
            let abs_days = if self.days < 0 { -self.days } else { self.days };
            parts.push(format!("{} {}", abs_days, get_day_suffix(self.days)));
        }

        if self.hours != 0 {
            let abs_hours = if self.hours < 0 {
                -self.hours
            } else {
                self.hours
            };
            parts.push(format!("{} {}", abs_hours, get_hour_suffix(self.hours)));
        }

        if self.minutes != 0 {
            let abs_minutes = if self.minutes < 0 {
                -self.minutes
            } else {
                self.minutes
            };
            parts.push(format!("{} {}", abs_minutes, get_min_suffix(self.minutes)));
        }

        if self.seconds != 0 || self.microseconds != 0 {
            let abs_seconds = if self.seconds < 0 {
                -self.seconds
            } else {
                self.seconds
            };
            let abs_micros = if self.microseconds < 0 {
                -self.microseconds
            } else {
                self.microseconds
            };
            if abs_micros != 0 {
                let secs_with_micros = abs_seconds as f64 + abs_micros as f64 / 1_000_000.0;
                parts.push(format!(
                    "{} {}",
                    secs_with_micros,
                    get_sec_suffix(self.seconds)
                ));
            } else {
                parts.push(format!("{} {}", abs_seconds, get_sec_suffix(self.seconds)));
            }
        }

        if parts.is_empty() {
            return "@ 0".to_owned();
        }

        let result = format!("@ {}", parts.join(" "));
        if is_negative {
            format!("{} ago", result)
        } else {
            result
        }
    }
}
