use chrono::{NaiveDate, NaiveDateTime};
use mysql::{prelude::FromRow, Value};

#[derive(Debug, Default)]
pub struct SysUserTokenEntity {
    pub id: i64,
    pub user_id: i64,
    pub token: String,
    pub expire_time: NaiveDateTime,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

impl FromRow for SysUserTokenEntity {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError> {
        let (id, user_id, token, expire_time, create_time, update_time) = mysql::from_row(row);
        Ok(SysUserTokenEntity {
            id,
            user_id,
            token,
            expire_time: convert(expire_time),
            create_time: convert(create_time),
            update_time: convert(update_time),
        })
    }
}

fn convert(value: Value) -> NaiveDateTime {
    match value {
        Value::Date(y, m, d, h, i, s, us) => NaiveDate::from_ymd_opt(y as i32, m as u32, d as u32)
            .unwrap()
            .and_hms_micro_opt(h as u32, i as u32, s as u32, us),
        _ => NaiveDateTime::from_timestamp_opt(0, 0),
    }
    .unwrap()
}
