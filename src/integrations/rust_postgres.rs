use crate::Interval;
use bytes::{Buf, BufMut, BytesMut};
use postgres_types::{FromSql, IsNull, ToSql, Type, to_sql_checked};
use std::error::Error;

impl<'a> FromSql<'a> for Interval {
    fn from_sql(_: &Type, mut raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let microseconds = raw.get_i64();
        let days = raw.get_i32();
        let months = raw.get_i32();
        Ok(Interval {
            months,
            days,
            microseconds,
        })
    }

    fn accepts(ty: &Type) -> bool {
        matches!(*ty, Type::INTERVAL)
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
        matches!(*ty, Type::INTERVAL)
    }

    to_sql_checked!();
}
