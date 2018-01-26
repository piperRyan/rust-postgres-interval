# Rust-Postgres-Interval
A interval type for the postgres driver.

## Overview
Rust-Postgres-Interval is dedicated datatype for the postgres interval type.

```rust
extern crate postgres_interval;

use postgres_interval::Interval;

fn main() {
    let months = 13;
    let days = 1;
    let microseconds = 3600000000;
    let interval = Interval::new(months, days, microseconds);
    let output = interval.to_iso_8601();
    assert_eq!(String::from("P1Y1M1DT1H"), output);
}
```

## Requirements
- rust 1.22
