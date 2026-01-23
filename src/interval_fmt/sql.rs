use crate::interval_norm::IntervalNorm;

impl IntervalNorm {
    pub fn into_sql(self) -> String {
        let has_negative = self.years < 0
            || self.months < 0
            || self.days < 0
            || self.hours < 0
            || self.minutes < 0
            || self.seconds < 0
            || self.microseconds < 0;
        let has_positive = self.years > 0
            || self.months > 0
            || self.days > 0
            || self.hours > 0
            || self.minutes > 0
            || self.seconds > 0
            || self.microseconds > 0;
        let has_year_month = self.years != 0 || self.months != 0;
        let has_day_time = self.days != 0
            || self.hours != 0
            || self.minutes != 0
            || self.seconds != 0
            || self.microseconds != 0;
        let has_day = self.days != 0;
        let is_year_month_only = has_year_month && !has_day_time;
        let is_day_time_only = has_day_time && !has_year_month;
        let sql_standard_value =
            !(has_negative && has_positive) && !(has_year_month && has_day_time);

        if !has_negative && !has_positive {
            return "0".to_owned();
        }

        if !sql_standard_value {
            // Mixed signs or year-month + day-time: force signs on each component
            let year_sign = if self.years < 0 || self.months < 0 {
                '-'
            } else {
                '+'
            };
            let day_sign = if self.days < 0 { '-' } else { '+' };
            let sec_sign = if self.hours < 0
                || self.minutes < 0
                || self.seconds < 0
                || self.microseconds < 0
            {
                '-'
            } else {
                '+'
            };

            let (years, months, days, hours, minutes, seconds, microseconds) = if year_sign == '-' {
                (
                    -self.years,
                    -self.months,
                    -self.days,
                    -self.hours,
                    -self.minutes,
                    -self.seconds,
                    -self.microseconds,
                )
            } else {
                (
                    self.years,
                    self.months,
                    self.days,
                    self.hours,
                    self.minutes,
                    self.seconds,
                    self.microseconds,
                )
            };

            let hours_abs = super::safe_abs_u64(hours);
            let minutes_abs = super::safe_abs_u64(minutes);
            let seconds_abs = super::safe_abs_u64(seconds);
            let mut time_str = format!(
                "{}{}:{:02}:{:02}",
                sec_sign, hours_abs, minutes_abs, seconds_abs
            );

            if microseconds != 0 {
                let microseconds_fmt = format!(".{:06}", super::safe_abs_u64(microseconds));
                time_str.push_str(&microseconds_fmt);
            }

            format!(
                "{}{}-{} {}{} {}",
                year_sign,
                years.abs(),
                months.abs(),
                day_sign,
                days.abs(),
                time_str
            )
        } else if is_year_month_only {
            // Only year-month
            if has_negative {
                format!("-{}-{}", self.years.abs(), super::safe_abs_u32(self.months))
            } else {
                format!("{}-{}", self.years, super::safe_abs_u32(self.months))
            }
        } else if is_day_time_only && has_day {
            // Day-time with day
            let hours_abs = super::safe_abs_u64(self.hours);
            let minutes = super::safe_abs_u64(self.minutes);
            let seconds = super::safe_abs_u64(self.seconds);
            let mut time_str = if self.hours == 0
                && self.minutes == 0
                && self.seconds == 0
                && self.microseconds == 0
            {
                "0:00:00".to_owned()
            } else {
                format!("{}:{:02}:{:02}", hours_abs, minutes, seconds)
            };

            if self.microseconds != 0 {
                let microseconds_fmt = format!(".{:06}", super::safe_abs_u64(self.microseconds));
                time_str.push_str(&microseconds_fmt);
            }

            if has_negative {
                format!("-{} {}", self.days.abs(), time_str)
            } else {
                format!("{} {}", self.days, time_str)
            }
        } else if is_day_time_only {
            // Time only
            let hours_abs = super::safe_abs_u64(self.hours);
            let minutes = super::safe_abs_u64(self.minutes);
            let seconds = super::safe_abs_u64(self.seconds);
            let sign = if self.hours < 0 { "-" } else { "" };
            let mut result = format!("{}{}:{:02}:{:02}", sign, hours_abs, minutes, seconds);

            if self.microseconds != 0 {
                let microseconds_fmt = format!(".{:06}", super::safe_abs_u64(self.microseconds));
                result.push_str(&microseconds_fmt);
            }

            result
        } else {
            "0".to_owned()
        }
    }
}
