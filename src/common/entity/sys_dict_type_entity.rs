use chrono::{NaiveDate, NaiveDateTime};
use mysql::{prelude::FromRow, Value};

use crate::modules::dto::sys_dict_type_dto::SysDictTypeDto;

#[derive(Debug, Default)]
pub struct SysDictTypeEntity {
    pub id: i64,
    pub dict_type: String,
    pub dict_name: String,
    pub remark: String,
    pub sort: i64,
    pub creator: i64,
    pub create_date: NaiveDateTime,
    pub updater: i64,
    pub update_date: NaiveDateTime,
}

impl SysDictTypeEntity {
    pub fn from(dto: &SysDictTypeDto) -> Self {
        SysDictTypeEntity {
            id: dto.id,
            dict_name: dto.dict_name.clone(),
            dict_type: dto.dict_type.clone(),
            remark: dto.remark.clone(),
            sort: dto.sort,
            creator: 0,
            create_date: dto.create_date,
            updater: 0,
            update_date: dto.update_date,
        }
    }
}

impl FromRow for SysDictTypeEntity {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError> {
        let (id, dict_type, dict_name, remark, sort, creator, create_date, updater, update_date) =
            mysql::from_row(row);
        Ok(SysDictTypeEntity {
            id,
            dict_type,
            dict_name,
            remark,
            sort,
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
