use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{common::entity::sys_menu_entity::SysMenuEntity, modules::my_date_format};

use serde_with::{serde_as, DisplayFromStr};

use super::TreeNode;

#[serde_as]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct SysMenuDto {
    // serialize to string
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    #[serde_as(as = "DisplayFromStr")]
    pub pid: i64, // 父菜单ID，一级菜单为0
    pub name: String,                // 菜单名称
    pub url: Option<String>,         // 菜单URL
    pub menu_type: i32,              // 类型  0：菜单   1：按钮
    pub icon: Option<String>,        // 菜单图标
    pub sort: i32,                   // 排序
    pub permissions: Option<String>, // 授权(多个用逗号分隔，如：user:list,user:create)
    #[serde(with = "my_date_format")]
    pub create_date: NaiveDateTime,
    pub parent_name: Option<String>,
    pub children: Vec<SysMenuDto>,
}

impl SysMenuDto {
    pub fn from(menu: SysMenuEntity) -> SysMenuDto {
        SysMenuDto {
            id: menu.id,
            pid: menu.pid,
            name: menu.name,
            url: menu.url,
            menu_type: menu.menu_type,
            icon: menu.icon,
            sort: menu.sort,
            permissions: menu.perms,
            create_date: menu.create_time,
            parent_name: menu.parent_name,
            children: vec![],
        }
    }
}

impl TreeNode for SysMenuDto {
    fn get_id(&self) -> i64 {
        self.id
    }

    fn get_pid(&self) -> i64 {
        self.pid
    }

    fn add_child(&mut self, child: Self) {
        self.children.push(child);
    }

    fn get_children(&self) -> &Vec<SysMenuDto> {
        &self.children
    }

    fn get_children_mut(&mut self) -> &mut Vec<SysMenuDto> {
        &mut self.children
    }
}
