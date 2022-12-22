use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DictData {
    pub dict_type_id: i64,
    pub dict_label: String,
    pub dict_value: String,
}
