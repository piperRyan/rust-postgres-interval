mod iso_8601;
mod postgres;
pub mod parse_error;

static DAYS_PER_MONTH: i32 = 30;
static MONTHS_PER_YEAR: i32 = 12;
static SECONDS_PER_MIN: i32 = 60;
static HOURS_PER_DAY: i32 = 24;
static MINUTES_PER_HOUR: i32 = 60;
static MICROS_PER_SECOND: i32 = 1_000_000;




fn scale_date(val: f64, scale: i32) -> (i32, i32) {
    if val.fract() == 0.0 {
        return (val.trunc() as i32, 0)
    } else {
        // matches postgres implementation of just truncating.
        let sub_value = (val.fract() * scale as f64).round() as i32;
        (val.trunc() as i32, sub_value)
    }
}

fn scale_time(val: f64, scale: i32) -> (i64, i64) {
      if val.fract() == 0.0 {
        return (val.trunc() as i64, 0)
    } else {
        // matches postgres implementation of just truncating.
        let sub_value = (val.fract() * scale as f64).round() as i64;
        (val.trunc() as i64, sub_value)
    }
} 