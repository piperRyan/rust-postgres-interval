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
