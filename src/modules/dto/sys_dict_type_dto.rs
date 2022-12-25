use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use crate::{common::entity::sys_dict_type_entity::SysDictTypeEntity, modules::my_date_format};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct SysDictTypeDto {
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    pub dict_type: String,
    pub dict_name: String,
    pub remark: String,
    // #[serde_as(as = "DisplayFromStr")]
    pub sort: i64,
    #[serde(with = "my_date_format")]
    pub create_date: NaiveDateTime,
    #[serde(with = "my_date_format")]
    pub update_date: NaiveDateTime,
}

impl SysDictTypeDto {
    pub fn from(entity: &SysDictTypeEntity) -> Self {
        SysDictTypeDto {
            id: entity.id,
            dict_name: entity.dict_name.clone(),
            dict_type: entity.dict_type.clone(),
            remark: entity.remark.clone(),
            sort: entity.sort,
            create_date: entity.create_date,
            update_date: entity.update_date,
        }
    }
}
