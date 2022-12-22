use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{common::entity::sys_user_entity::SysUserEntity, modules::my_date_format};

use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct SysUserDto {
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    pub username: String,
    pub password: String,
    pub real_name: String,
    pub head_url: Option<String>,
    pub gender: i32,
    pub email: String,
    pub mobile: String,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub dept_id: Option<i64>,
    pub status: i32, // 0:禁用 1:正常
    #[serde(with = "my_date_format")]
    pub create_date: NaiveDateTime,
    pub super_admin: i32,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub role_id_list: Vec<i64>,
    pub dept_name: Option<String>,
}

impl SysUserDto {
    pub fn from(user: Option<SysUserEntity>) -> Option<SysUserDto> {
        match user {
            Some(user) => Some(SysUserDto {
                id: user.id,
                username: user.username,
                password: user.password,
                real_name: user.real_name,
                head_url: user.head_url,
                gender: user.gender,
                email: user.email,
                mobile: user.mobile,
                dept_id: user.dept_id,
                status: 1,
                create_date: user.create_time,
                super_admin: user.super_admin,
                role_id_list: vec![],
                dept_name: user.dept_name,
            }),
            None => None,
        }
    }
}
