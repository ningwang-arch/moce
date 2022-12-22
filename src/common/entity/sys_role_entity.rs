use chrono::{NaiveDate, NaiveDateTime};
use mysql::{prelude::FromRow, Value};
use serde::{Deserialize, Serialize};

use crate::modules::dto::sys_role_dto::SysRoleDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysRoleEntity {
    pub id: i64,
    pub name: String,
    pub remark: Option<String>,
    pub dept_id: Option<i64>,
    pub creator: i64,
    pub create_date: NaiveDateTime,
    pub updater: i64,
    pub update_date: NaiveDateTime,
}

impl SysRoleEntity {
    pub fn from(dto: SysRoleDto) -> Self {
        SysRoleEntity {
            id: dto.id,
            name: dto.name,
            remark: dto.remark,
            dept_id: None,
            creator: 0,
            create_date: chrono::Local::now().naive_local(),
            updater: 0,
            update_date: chrono::Local::now().naive_local(),
        }
    }
}

impl FromRow for SysRoleEntity {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        let (id, name, remark, dept_id, creator, create_date, updater, update_date) =
            mysql::from_row(row);
        Ok(SysRoleEntity {
            id,
            name,
            remark,
            dept_id,
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
