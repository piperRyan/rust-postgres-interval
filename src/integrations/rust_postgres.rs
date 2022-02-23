use crate::Interval;
use bytes::{Buf, BufMut, BytesMut};
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};

impl<'a> FromSql<'a> for Interval {
    fn from_sql(_: &Type, mut raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
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
