[![Build Status](https://travis-ci.org/piperRyan/rust-postgres-interval.svg?branch=master)](https://travis-ci.org/piperRyan/rust-postgres-interval)

[![codecov](https://codecov.io/gh/piperRyan/rust-postgres-interval/branch/master/graph/badge.svg)](https://codecov.io/gh/piperRyan/rust-postgres-interval)

# Rust-Postgres-Interval
A interval type for the postgres driver.

# Contributing

There is a separate document on how to contribute to this repo [here](CONTRIBUTING.md)

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

## Roadmap to 1.0.0

- [ ] Implement method to covert interval into a sql compliant interval string
- [ ] Parse iso 8601, postgre, and sql interval strings into interval
- [ ] Support for nightly and beta
- [ ] Poll community to better define use cases
- [ ] Basic chrono integration 
