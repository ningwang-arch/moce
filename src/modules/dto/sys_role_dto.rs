use chrono::NaiveDateTime;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use crate::{common::entity::sys_role_entity::SysRoleEntity, modules::my_date_format};

#[serde_as]
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct SysRoleDto {
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    pub name: String,
    pub remark: Option<String>,
    #[serde(with = "my_date_format")]
    pub create_date: NaiveDateTime,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub menu_id_list: Vec<i64>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub dept_id_list: Vec<i64>,
}

impl SysRoleDto {
    pub fn from(role: &SysRoleEntity) -> Self {
        SysRoleDto {
            id: role.id,
            name: role.name.clone(),
            remark: role.remark.clone(),
            create_date: role.create_date,
            menu_id_list: vec![],
            dept_id_list: vec![],
        }
    }
}
