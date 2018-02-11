extern crate postgres_interval;
use postgres_interval::Interval;

#[test]
fn test_iso_1() {
    let interval = Interval::new(12,0,0);
    let output = interval.to_iso_8601();
    assert_eq!(String::from("P1Y"), output);
}

#[test]
fn test_8601_2() {
    let interval = Interval::new(13,0,0);
    let output = interval.to_iso_8601();
    assert_eq!(String::from("P1Y1M"), output);
}

#[test]
fn test_8601_3() {
    let interval = Interval::new(13,1,0);
    let output = interval.to_iso_8601();
    assert_eq!(String::from("P1Y1M1D"), output);
}

#[test]
fn test_8601_4() {
    let interval = Interval::new(13,1,3600000000);
    let output = interval.to_iso_8601();
    assert_eq!(String::from("P1Y1M1DT1H"), output);
}

#[test]
fn test_8601_5() {
    let interval = Interval::new(13,1,4200000000);
    let output = interval.to_iso_8601();
    assert_eq!(String::from("P1Y1M1DT1H10M"), output);
}


#[test]
fn test_8601_6() {
    let interval = Interval::new(13,1,4215000000);
    let output = interval.to_iso_8601();
    assert_eq!(String::from("P1Y1M1DT1H10M15S"), output);
}

#[test]
fn test_8601_7() {
    let interval = Interval::new(0,0, 3600000000);
    let output = interval.to_iso_8601();
    assert_eq!(String::from("PT1H"), output);
}

#[test]
fn test_8601_8() {
    let interval = Interval::new(0,0,4200000000);
    let output = interval.to_iso_8601();
    assert_eq!(String::from("PT1H10M"), output);
}

#[test]
fn test_8601_9() {
    let interval = Interval::new(0,0,4215000000);
    let output = interval.to_iso_8601();
    assert_eq!(String::from("PT1H10M15S"), output);
}

#[test]
fn test_8601_10() {
    let interval = Interval::new(-12,0,0);
    let output = interval.to_iso_8601();
    assert_eq!(String::from("P-1Y"), output);
}

#[test]
fn test_8601_11() {
    let interval = Interval::new(-13,0,0);
    let output = interval.to_iso_8601();
    assert_eq!(String::from("P-1Y-1M"), output);
}

#[test]
fn test_8601_12() {
    let interval = Interval::new(-13,-1,0);
    let output = interval.to_iso_8601();
    assert_eq!(String::from("P-1Y-1M-1D"), output);
}

#[test]
fn test_8601_13() {
    let interval = Interval::new(-13,-1,-3600000000);
    let output = interval.to_iso_8601();
    assert_eq!(String::from("P-1Y-1M-1DT-1H"), output);
}

#[test]
fn test_8601_14() {
    let interval = Interval::new(-13,-1,-4200000000);
    let output = interval.to_iso_8601();
    assert_eq!(String::from("P-1Y-1M-1DT-1H-10M"), output);
}


#[test]
fn test_8601_15() {
    let interval = Interval::new(-13,-1,-4215000000);
    let output = interval.to_iso_8601();
    assert_eq!(String::from("P-1Y-1M-1DT-1H-10M-15S"), output);
}

#[test]
fn test_8601_16() {
    let interval = Interval::new(0,0, -3600000000);
    let output = interval.to_iso_8601();
    assert_eq!(String::from("PT-1H"), output);
}

#[test]
fn test_8601_17() {
    let interval = Interval::new(0,0,-4200000000);
    let output = interval.to_iso_8601();
    assert_eq!(String::from("PT-1H-10M"), output);
}

#[test]
fn test_8601_18() {
    let interval = Interval::new(0,0,-4215000000);
    let output = interval.to_iso_8601();
    assert_eq!(String::from("PT-1H-10M-15S"), output);
}

#[test]
fn test_postgres_1() {
    let interval = Interval::new(12,0,0);
    let output = interval.to_postgres();
    assert_eq!(String::from("1 year"), output);
}

#[test]
fn test_postgres_2() {
    let interval = Interval::new(13,0,0);
    let output = interval.to_postgres();
    assert_eq!(String::from("1 year 1 mons"), output);
}

#[test]
fn test_postgres_3() {
    let interval = Interval::new(13,1,0);
    let output = interval.to_postgres();
    assert_eq!(String::from("1 year 1 mons 1 days"), output);
}

#[test]
fn test_postgres_4() {
    let interval = Interval::new(13,1,3600000000);
    let output = interval.to_postgres();
    assert_eq!(String::from("1 year 1 mons 1 days 01:00:00"), output);
}

#[test]
fn test_postgres_5() {
    let interval = Interval::new(13,1,4200000000);
    let output = interval.to_postgres();
    assert_eq!(String::from("1 year 1 mons 1 days 01:10:00"), output);
}

#[test]
fn test_postgres_6() {
    let interval = Interval::new(13,1,4215000000);
    let output = interval.to_postgres();
    assert_eq!(String::from("1 year 1 mons 1 days 01:10:15"), output);
}

#[test]
fn test_postgres_7() {
    let interval = Interval::new(0,0, 3600000000);
    let output = interval.to_postgres();
    assert_eq!(String::from("01:00:00"), output);
}

#[test]
fn test_postgres_8() {
    let interval = Interval::new(0,0,4200000000);
    let output = interval.to_postgres();
    assert_eq!(String::from("01:10:00"), output);
}

#[test]
fn test_postgres_9() {
    let interval = Interval::new(0,0,4215000000);
    let output = interval.to_postgres();
    assert_eq!(String::from("01:10:15"), output);
}

#[test]
fn test_postgres_10() {
    let interval = Interval::new(-12,0,0);
    let output = interval.to_postgres();
    assert_eq!(String::from("-1 year"), output);
}

#[test]
fn test_postgres_11() {
    let interval = Interval::new(-13,0,0);
    let output = interval.to_postgres();
    assert_eq!(String::from("-1 year -1 mons"), output);
}

#[test]
fn test_postgres_12() {
    let interval = Interval::new(-13,-1,0);
    let output = interval.to_postgres();
    assert_eq!(String::from("-1 year -1 mons -1 days"), output);
}

#[test]
fn test_postgres_13() {
    let interval = Interval::new(-13,-1,-3600000000);
    let output = interval.to_postgres();
    assert_eq!(String::from("-1 year -1 mons -1 days -01:00:00"), output);
}

#[test]
fn test_postgres_14() {
    let interval = Interval::new(-13,-1,-4200000000);
    let output = interval.to_postgres();
    assert_eq!(String::from("-1 year -1 mons -1 days -01:10:00"), output);
}


#[test]
fn test_postgres_16() {
    let interval = Interval::new(0,0, -3600000000);
    let output = interval.to_postgres();
    assert_eq!(String::from("-01:00:00"), output);
}

#[test]
fn test_postgres_17() {
    let interval = Interval::new(0,0,-4200000000);
    let output = interval.to_postgres();
    assert_eq!(String::from("-01:10:00"), output);
}

#[test]
fn test_postgres_18() {
    let interval = Interval::new(0,0,-4215000000);
    let output = interval.to_postgres();
    assert_eq!(String::from("-01:10:15"), output);
}

#[test]
fn test_checked_add() {
    let interval = Interval::new(13,0,0);
    let interval_add = Interval::new(2,1,12);
    let result = interval.checked_add(interval_add);
    assert_eq!(result, Some(Interval::new(15,1,12)));
}

#[test]
fn test_checked_add_2() {
    let interval = Interval::new(13,0,0);
    let interval_add = Interval::new(i32::max_value(), 1, 12);
    let result = interval.checked_add(interval_add);
    assert_eq!(result, None);
}

#[test]
fn test_checked_sub() {
    let interval = Interval::new(13,0,0);
    let interval_sub = Interval::new(2,0,0);
    let result = interval.checked_sub(interval_sub);
    assert_eq!(result, Some(Interval::new(11,0,0)));
}

#[test]
fn test_checked_sub_2() {
    let interval = Interval::new(-10,0,0);
    let interval_sub = Interval::new(i32::max_value(), 0, 0);
    let result = interval.checked_sub(interval_sub);
    assert_eq!(result, None);
}

#[test]
fn test_add_day_time() {
    let interval = Interval::new(13,0,0);
    let result = interval.add_day_time(2,0,0, 2.123456789);
    assert_eq!(result, Interval::new(13,2,2123456));
}

#[test]
fn test_sub_day_time() {
    let interval = Interval::new(13,0,0);
    let result = interval.sub_day_time(2,0,0, 2.12);
    assert_eq!(result, Interval::new(13,-2,-2120000));
}

#[test]
fn test_checked_add_day_time() {
    let interval = Interval::new(13,0,0);
    let result = interval.checked_add_day_time(2,0,0, 2.123456789);
    assert_eq!(result, Some(Interval::new(13,2,2123456)));
}

#[test]
fn test_checked_add_day_time_2() {
    let interval = Interval::new(13,i32::max_value(),0);
    let result = interval.checked_add_day_time(200,0,0, 2.123456789);
    assert_eq!(result, None);
}

#[test]
fn test_checked_sub_day_time() {
    let interval = Interval::new(13,0,0);
    let result = interval.checked_sub_day_time(2,0,0, 2.12);
    assert_eq!(result, Some(Interval::new(13,-2,-2120000)));
}

#[test]
fn test_checked_sub_day_time_2() {
    let interval = Interval::new(13,i32::min_value(),0);
    println!("{:?}", interval.days());
    let result = interval.checked_sub_day_time(100,0,0, 2.12);
    println!("{:?}", result);
    assert_eq!(result, None);
}
