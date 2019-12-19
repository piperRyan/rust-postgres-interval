use std::error::Error;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use postgres::types::{FromSql, IsNull, ToSql, Type, INTERVAL};

use crate::Interval;

impl FromSql for Interval {
    fn from_sql(_: &Type, mut raw: &[u8]) -> Result<Interval, Box<dyn Error + Sync + Send>> {
        let microseconds: i64 = raw.read_i64::<BigEndian>()?;
        let days: i32 = raw.read_i32::<BigEndian>()?;
        let months: i32 = raw.read_i32::<BigEndian>()?;

        Ok(Interval {
            months,
            days,
            microseconds,
        })
    }

    fn accepts(ty: &Type) -> bool {
        match *ty {
            INTERVAL => true,
            _ => false,
        }
    }
}

impl ToSql for Interval {
    fn to_sql(&self, _: &Type, out: &mut Vec<u8>) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        out.write_i64::<BigEndian>(self.microseconds)?;
        out.write_i32::<BigEndian>(self.days)?;
        out.write_i32::<BigEndian>(self.months)?;

        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        match *ty {
            INTERVAL => true,
            _ => false,
        }
    }

    postgres::to_sql_checked!();
}
