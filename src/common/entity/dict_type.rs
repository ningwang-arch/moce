use serde::{Deserialize, Serialize};

use crate::common::entity::dict_data::DictData;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DictType {
    pub id: i64,
    pub dict_type: String,
    pub data_list: Vec<DictData>,
}
