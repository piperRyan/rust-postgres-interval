#[cfg(feature = "postgres")]
#[macro_use]
extern crate postgres;
#[cfg(feature = "postgres")]
extern crate byteorder;

#[cfg(feature = "postgres")]
mod integrations;

mod interval_fmt;
mod interval_norm;
mod pg_interval;
mod pg_interval_add;
mod pg_interval_sub;
pub use pg_interval::Interval;
