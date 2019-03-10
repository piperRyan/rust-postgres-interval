mod iso_8601;
mod postgres;
mod sql;

use std::ops::Neg;

/// Safely maps a i64 value to a unsigned number
/// without any overflow issues.
fn safe_abs_u64(mut num: i64) -> u64 {
    let max = i64::max_value();
    let max_min = max.neg();
    if num <= max_min {
        let result = max as u64;
        num += max;
        num *= -1;
        result + num as u64
    } else {
        num.abs() as u64
    }
}

/// Safely maps a i32 value to a unsigned number
/// without any overflow issues.
fn safe_abs_u32(mut num: i32) -> u32 {
    let max = i32::max_value();
    let max_min = max.neg();
    if num <= max_min {
        let result = max as u32;
        num += max;
        num *= -1;
        result + num as u32
    } else {
        num.abs() as u32
    }
}

/// Pads a i64 value with a width of 2.
fn pad_i64(val: i64) -> String {
    let num = if val < 0 {
        safe_abs_u64(val)
    } else {
        val as u64
    };
    return format!("{:02}", num);
}

#[cfg(test)]
mod tests {
    use super::*;
    
     #[test]
    fn abs_safe_u32() {
        let min = i32::min_value();
        let actual = safe_abs_u32(min); 
        let expected = 2147483648;
        assert_eq!(actual, expected);
    }

      #[test]
    fn abs_safe_u64() {
        let min = i64::min_value();
        let actual = safe_abs_u64(min); 
        let expected = 9_223_372_036_854_775_808;
        assert_eq!(actual, expected);
    }
}