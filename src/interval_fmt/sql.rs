use crate::interval_norm::IntervalNorm;

impl IntervalNorm {
    fn format_time(
        hours: i64,
        minutes: i64,
        seconds: i64,
        microseconds: i64,
        sign: bool,
    ) -> String {
        let sign_str = if sign { "-" } else { "" };
        let time_str = format!(
            "{}{}:{:02}:{:02}",
            sign_str,
            super::safe_abs_u64(hours),
            super::safe_abs_u64(minutes),
            super::safe_abs_u64(seconds)
        );

        if microseconds != 0 {
            format!("{}.{:06}", time_str, super::safe_abs_u64(microseconds))
        } else {
            time_str
        }
    }

    pub fn into_sql(self) -> String {
        let has_negative = self.has_negative();
        let has_positive = self.has_positive();

        let has_year_month = self.is_year_month_present();
        let has_day_time = self.is_day_present() || self.is_time_present();

        let sql_standard_value = !(has_negative && has_positive || has_year_month && has_day_time);

        if !has_negative && !has_positive {
            return "0".to_owned();
        }

        if !sql_standard_value {
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

            let time_str = format!(
                "{}{}:{:02}:{:02}",
                sec_sign,
                super::safe_abs_u64(self.hours),
                super::safe_abs_u64(self.minutes),
                super::safe_abs_u64(self.seconds)
            );

            let time_str = if self.microseconds != 0 {
                format!("{}.{:06}", time_str, super::safe_abs_u64(self.microseconds))
            } else {
                time_str
            };

            format!(
                "{}{}-{} {}{} {}",
                year_sign,
                self.years.abs(),
                self.months.abs(),
                day_sign,
                self.days.abs(),
                time_str
            )
        } else if has_year_month {
            format!("{}-{}", self.years, super::safe_abs_u32(self.months))
        } else if self.days != 0 {
            format!(
                "{} {}",
                self.days,
                Self::format_time(
                    self.hours,
                    self.minutes,
                    self.seconds,
                    self.microseconds,
                    false
                )
            )
        } else {
            Self::format_time(
                self.hours,
                self.minutes,
                self.seconds,
                self.microseconds,
                has_negative,
            )
        }
    }
}
