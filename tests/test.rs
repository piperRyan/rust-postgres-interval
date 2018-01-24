extern crate postgres_interval;
use postgres_interval::Interval;

#[test]
fn test_8601_1() {
    let interval = Interval::new(12,0,0);
    let iso_string = interval.to_iso_8601();
    assert_eq!(String::from("P1Y"), iso_string);
}

#[test]
fn test_8601_2() {
    let interval = Interval::new(13,0,0);
    let iso_string = interval.to_iso_8601();
    assert_eq!(String::from("P1Y1M"), iso_string);
}

#[test]
fn test_8601_3() {
    let interval = Interval::new(13,1,0);
    let iso_string = interval.to_iso_8601();
    assert_eq!(String::from("P1Y1M1D"), iso_string);
}

#[test]
fn test_8601_4() {
    let interval = Interval::new(13,1,3600000000);
    let iso_string = interval.to_iso_8601();
    assert_eq!(String::from("P1Y1M1DT1H"), iso_string);
}

#[test]
fn test_8601_5() {
    let interval = Interval::new(13,1,4200000000);
    let iso_string = interval.to_iso_8601();
    assert_eq!(String::from("P1Y1M1DT1H10M"), iso_string);
}


#[test]
fn test_8601_6() {
    let interval = Interval::new(13,1,4215000000);
    let iso_string = interval.to_iso_8601();
    assert_eq!(String::from("P1Y1M1DT1H10M15S"), iso_string);
}

#[test]
fn test_8601_7() {
    let interval = Interval::new(0,0, 3600000000);
    let iso_string = interval.to_iso_8601();
    assert_eq!(String::from("PT1H"), iso_string);
}

#[test]
fn test_8601_8() {
    let interval = Interval::new(0,0,4200000000);
    let iso_string = interval.to_iso_8601();
    assert_eq!(String::from("PT1H10M"), iso_string);
}

#[test]
fn test_8601_9() {
    let interval = Interval::new(0,0,4215000000);
    let iso_string = interval.to_iso_8601();
    assert_eq!(String::from("PT1H10M15S"), iso_string);
}

#[test]
fn test_8601_10() {
    let interval = Interval::new(-12,0,0);
    let iso_string = interval.to_iso_8601();
    assert_eq!(String::from("P-1Y"), iso_string);
}

#[test]
fn test_8601_11() {
    let interval = Interval::new(-13,0,0);
    let iso_string = interval.to_iso_8601();
    assert_eq!(String::from("P-1Y-1M"), iso_string);
}

#[test]
fn test_8601_12() {
    let interval = Interval::new(-13,-1,0);
    let iso_string = interval.to_iso_8601();
    assert_eq!(String::from("P-1Y-1M-1D"), iso_string);
}

#[test]
fn test_8601_13() {
    let interval = Interval::new(-13,-1,-3600000000);
    let iso_string = interval.to_iso_8601();
    assert_eq!(String::from("P-1Y-1M-1DT-1H"), iso_string);
}

#[test]
fn test_8601_14() {
    let interval = Interval::new(-13,-1,-4200000000);
    let iso_string = interval.to_iso_8601();
    assert_eq!(String::from("P-1Y-1M-1DT-1H-10M"), iso_string);
}


#[test]
fn test_8601_15() {
    let interval = Interval::new(-13,-1,-4215000000);
    let iso_string = interval.to_iso_8601();
    assert_eq!(String::from("P-1Y-1M-1DT-1H-10M-15S"), iso_string);
}

#[test]
fn test_8601_16() {
    let interval = Interval::new(0,0, -3600000000);
    let iso_string = interval.to_iso_8601();
    assert_eq!(String::from("PT-1H"), iso_string);
}

#[test]
fn test_8601_17() {
    let interval = Interval::new(0,0,-4200000000);
    let iso_string = interval.to_iso_8601();
    assert_eq!(String::from("PT-1H-10M"), iso_string);
}

#[test]
fn test_8601_18() {
    let interval = Interval::new(0,0,-4215000000);
    let iso_string = interval.to_iso_8601();
    assert_eq!(String::from("PT-1H-10M-15S"), iso_string);
}
