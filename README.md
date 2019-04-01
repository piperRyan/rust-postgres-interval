[![Build Status](https://travis-ci.org/piperRyan/rust-postgres-interval.svg?branch=master)](https://travis-ci.org/piperRyan/rust-postgres-interval) [![codecov](https://codecov.io/gh/piperRyan/rust-postgres-interval/branch/master/graph/badge.svg)](https://codecov.io/gh/piperRyan/rust-postgres-interval)

# Rust-Postgres-Interval
A interval type for the postgres driver.

# Contributing

There is a separate document on how to contribute to this repo [here](CONTRIBUTING.md)

## Overview
Rust-Postgres-Interval is dedicated datatype for the postgres interval type.

```rust
extern crate pg_interval;

use pg_interval::Interval;

fn main() {
    let interval = Interval::from_postgres(
        "1 years 1 months 1 days 1 hours"
    ).unwrap();
    let output = interval.to_iso_8601();
    assert_eq!(String::from("P1Y1M1DT1H"), output);
}
```

## Requirements
- rust 1.22

## Roadmap to 1.0.0

- [x] Convert Interval Into Formated String
    - [x] Iso 8601
    - [x] Postgres
    - [x] Sql
- [ ] Parse Formated Strings Into The Interval Type
    - [x] Iso 8601
    - [x] Postgres
    - [ ] Sql
- [ ] Chrono Integrations
