use crate::{interval_norm::IntervalNorm, Interval};
use chrono::Duration;

const NANOS_PER_SEC: i64 = 1_000_000_000;
const NANOS_PER_MICRO: i64 = 1000;

impl Interval {
    /// Tries to convert from the `Duration` type to a `Interval`. Will
    /// return `None` on a overflow. This is a lossy conversion in that
    /// any units smaller than a microsecond will be lost.   
    pub fn from_duration(duration: Duration) -> Option<Interval> {
        let mut days = duration.num_days();
        let mut new_dur = duration - Duration::days(days);
        let mut hours = duration.num_hours();
        new_dur = new_dur - Duration::hours(hours);
        let minutes = new_dur.num_minutes();
        new_dur = new_dur - Duration::minutes(minutes);
        let nano_secs = new_dur.num_nanoseconds()?;
        if days > (i32::max_value() as i64) {
            let overflow_days = days - (i32::max_value() as i64);
            let added_hours = overflow_days.checked_mul(24)?;
            hours = hours.checked_add(added_hours)?;
            days -= overflow_days;
        }
        let (seconds, remaining_nano) = reduce_by_units(nano_secs, NANOS_PER_SEC);
        // We have to discard any remaining nanoseconds
        let (microseconds, _remaining_nano) = reduce_by_units(remaining_nano, NANOS_PER_MICRO);
        let norm_interval = IntervalNorm {
            years: 0,
            months: 0,
            days: days as i32,
            hours,
            minutes,
            seconds,
            microseconds,
        };
        norm_interval.try_into_interval().ok()
    }
}

fn reduce_by_units(nano_secs: i64, unit: i64) -> (i64, i64) {
    let new_time_unit = (nano_secs - (nano_secs % unit)) / unit;
    let remaining_nano = nano_secs - (new_time_unit * unit);
    (new_time_unit, remaining_nano)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn can_convert_small_amount_of_days() {
        let dur = Duration::days(5);
        let interval = Interval::from_duration(dur);
        assert_eq!(interval, Some(Interval::new(0, 5, 0)))
    }

    #[test]
    fn overflow_on_days() {
        let dur = Duration::days(100000000000);
        let interval = Interval::from_duration(dur);
        assert_eq!(interval, None)
    }

    #[test]
    fn can_convert_small_amount_of_secs() {
        let dur = Duration::seconds(1);
        let interval = Interval::from_duration(dur);
        assert_eq!(interval, Some(Interval::new(0, 0, 1_000_000)))
    }

    #[test]
    fn can_convert_one_micro() {
        let dur = Duration::nanoseconds(1000);
        let interval = Interval::from_duration(dur);
        assert_eq!(interval, Some(Interval::new(0, 0, 1)))
    }
}
