use chrono::{NaiveDate, NaiveDateTime};
use mysql::{prelude::FromRow, Value};
use serde::{Deserialize, Serialize};

use crate::modules::dto::sys_dept_dto::SysDeptDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysDeptEntity {
    pub id: i64,
    pub pid: i64,                    // 父部门ID，一级部门为0
    pub pids: String,                // 所有上级部门ID
    pub name: String,                // 部门名称
    pub sort: i32,                   // 排序
    pub creator: i64,                // 创建者ID
    pub create_date: NaiveDateTime,  // 创建时间
    pub updater: i64,                // 更新者ID
    pub update_date: NaiveDateTime,  // 更新时间
    pub parent_name: Option<String>, // 上级部门名称
}

impl SysDeptEntity {
    pub fn from(dto: &SysDeptDto) -> SysDeptEntity {
        SysDeptEntity {
            id: dto.id,
            pid: dto.pid,
            pids: "".to_string(),
            name: dto.name.clone(),
            sort: dto.sort,
            creator: 0,
            create_date: dto.create_date,
            updater: 0,
            update_date: chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
            parent_name: dto.parent_name.clone(),
        }
    }
}

impl FromRow for SysDeptEntity {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        let (id, pid, pids, name, sort, creator, create_date, updater, update_date, parent_name) =
            mysql::from_row(row);
        Ok(SysDeptEntity {
            id,
            pid,
            pids,
            name,
            sort,
            creator,
            create_date: convert(create_date),
            updater,
            update_date: convert(update_date),
            parent_name,
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
