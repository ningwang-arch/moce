use chrono::{NaiveDate, NaiveDateTime};
use mysql::{prelude::FromRow, Value};

use crate::modules::dto::sys_params_dto::SysParamsDto;

#[derive(Debug, Default, Clone)]
pub struct SysParamsEntity {
    pub id: i64,
    pub param_code: String,
    pub param_value: String,
    pub param_type: i32,
    pub remark: String,
    pub creator: i64,
    pub create_date: NaiveDateTime,
    pub updater: i64,
    pub update_date: NaiveDateTime,
}

impl SysParamsEntity {
    pub fn from(dto: &SysParamsDto) -> SysParamsEntity {
        SysParamsEntity {
            id: dto.id,
            param_code: dto.param_code.clone(),
            param_value: dto.param_value.clone(),
            param_type: 1,
            remark: dto.remark.clone(),
            creator: 1,
            create_date: dto.create_date,
            updater: 1,
            update_date: dto.update_date,
        }
    }
}

impl FromRow for SysParamsEntity {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError> {
        let (
            id,
            param_code,
            param_value,
            param_type,
            remark,
            creator,
            create_date,
            updater,
            update_date,
        ) = mysql::from_row(row);
        Ok(SysParamsEntity {
            id,
            param_code,
            param_value,
            param_type,
            remark,
            creator,
            create_date: convert(create_date),
            updater,
            update_date: convert(update_date),
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
