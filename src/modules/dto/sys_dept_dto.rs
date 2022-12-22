use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use crate::{common::entity::sys_dept_entity::SysDeptEntity, modules::my_date_format};

use super::TreeNode;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct SysDeptDto {
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    #[serde_as(as = "DisplayFromStr")]
    pub pid: i64,
    pub name: String,
    pub sort: i32,
    #[serde(with = "my_date_format")]
    pub create_date: NaiveDateTime,
    pub parent_name: Option<String>,
    pub children: Vec<SysDeptDto>,
}

impl SysDeptDto {
    pub fn from(entity: &SysDeptEntity) -> SysDeptDto {
        SysDeptDto {
            id: entity.id,
            pid: entity.pid,
            name: entity.name.clone(),
            sort: entity.sort,
            create_date: entity.create_date,
            parent_name: entity.parent_name.clone(),
            children: vec![],
        }
    }
}

impl TreeNode for SysDeptDto {
    fn get_id(&self) -> i64 {
        self.id
    }

    fn get_pid(&self) -> i64 {
        self.pid
    }

    fn add_child(&mut self, child: Self) {
        self.children.push(child);
    }

    fn get_children(&self) -> &Vec<SysDeptDto> {
        &self.children
    }

    fn get_children_mut(&mut self) -> &mut Vec<SysDeptDto> {
        &mut self.children
    }
}
