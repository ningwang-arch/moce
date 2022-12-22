use chrono::{NaiveDate, NaiveDateTime};
use mysql::{prelude::FromRow, Value};
use serde::{Deserialize, Serialize};

use crate::modules::dto::sys_menu_dto::SysMenuDto;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SysMenuEntity {
    pub id: i64,
    pub pid: i64,              // 父菜单ID，一级菜单为0
    pub name: String,          // 菜单名称
    pub url: Option<String>,   // 菜单URL
    pub perms: Option<String>, // 授权(多个用逗号分隔，如：user:list,user:create)
    pub menu_type: i32,        // 类型  0：菜单   1：按钮
    pub icon: Option<String>,  // 菜单图标
    pub sort: i32,             // 排序
    pub creator: i64,          // 创建者
    pub create_time: NaiveDateTime,
    pub updater: i64, // 更新者
    pub update_time: NaiveDateTime,
    pub parent_name: Option<String>,
}

impl SysMenuEntity {
    pub fn from(dto: SysMenuDto) -> Self {
        SysMenuEntity {
            id: dto.id,
            pid: dto.pid,
            name: dto.name,
            url: dto.url,
            perms: dto.permissions,
            menu_type: dto.menu_type,
            icon: dto.icon,
            sort: dto.sort,
            creator: 0,
            create_time: dto.create_date,
            updater: 0,
            update_time: chrono::Local::now().naive_local(),
            parent_name: dto.parent_name,
        }
    }
}

impl FromRow for SysMenuEntity {
    fn from_row_opt(mut row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        Ok(SysMenuEntity {
            id: row.take("id").unwrap(),
            pid: row.take("pid").unwrap(),
            name: row.take("name").unwrap(),
            url: row.take("url").unwrap(),
            perms: row.take("permissions").unwrap(),
            menu_type: row.take("menu_type").unwrap(),
            icon: row.take("icon").unwrap(),
            sort: row.take("sort").unwrap(),
            creator: row.take("creator").unwrap(),
            create_time: convert(row.take("create_date").unwrap()),
            updater: row.take("updater").unwrap(),
            update_time: convert(row.take("update_date").unwrap()),
            parent_name: row.take("parent_name").unwrap(),
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
