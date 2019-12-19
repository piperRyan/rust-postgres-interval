use std::error::Error;

use byteorder::{BigEndian, ReadBytesExt};
use bytes::{BufMut, BytesMut};
use postgres_types::{FromSql, IsNull, ToSql, Type};

use crate::Interval;

impl<'a> FromSql<'a> for Interval {
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
        match ty {
            &Type::INTERVAL => true,
            _ => false,
        }
    }
}

impl ToSql for Interval {
    fn to_sql(&self, _: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        out.put_i64(self.microseconds);
        out.put_i32(self.days);
        out.put_i32(self.months);

        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        match ty {
            &Type::INTERVAL => true,
            _ => false,
        }
    }

    postgres_types::to_sql_checked!();
}
