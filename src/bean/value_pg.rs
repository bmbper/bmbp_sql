use crate::bean::value::RdbcValue;
use tokio_postgres::types::private::BytesMut;
use tokio_postgres::types::IsNull;
use tokio_postgres::types::{to_sql_checked, ToSql};

impl ToSql for RdbcValue {
    fn to_sql(
        &self,
        ty: &tokio_postgres::types::Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        match self {
            RdbcValue::Char(v) => v.to_string().to_sql(ty, w),
            RdbcValue::Varchar(v) => v.to_string().to_sql(ty, w),
            RdbcValue::Text(v) => v.to_string().to_sql(ty, w),
            RdbcValue::LongText(v) => v.to_string().to_sql(ty, w),
            RdbcValue::SmallInt(v) => (v.clone()).to_sql(ty, w),
            RdbcValue::Int(v) => (v.clone()).to_sql(ty, w),
            RdbcValue::BigInt(v) => v.clone().to_sql(ty, w),
            RdbcValue::Double(v) => v.clone().to_sql(ty, w),
            RdbcValue::BigDouble(v) => v.clone().to_sql(ty, w),
            RdbcValue::Date(v) => v.clone().to_sql(ty, w),
            RdbcValue::DateTime(v) => v.clone().to_sql(ty, w),
            RdbcValue::Time(v) => v.clone().to_sql(ty, w),
            RdbcValue::TimeStamp(v) => (v.clone() as i64).to_sql(ty, w),
            RdbcValue::Bytes(v) => v.to_vec().to_sql(ty, w),
            RdbcValue::Boolean(v) => v.clone().to_sql(ty, w),
            RdbcValue::Array(v) => v.to_vec().to_sql(ty, w),
            RdbcValue::Object(v) => match serde_json::to_string(v) {
                Ok(v) => v.to_sql(ty, w),
                Err(e) => e.to_string().to_sql(ty, w),
            },
            RdbcValue::Null => Ok(tokio_postgres::types::IsNull::Yes),
        }
    }
    fn accepts(_ty: &tokio_postgres::types::Type) -> bool
    where
        Self: Sized,
    {
        true
    }

    to_sql_checked!();
}
