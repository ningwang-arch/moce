use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use crate::modules::my_date_format;

#[serde_as]
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct SysParamsDto {
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    pub param_code: String,
    pub param_value: String,
    pub remark: String,
    #[serde(with = "my_date_format")]
    pub create_date: NaiveDateTime,
    #[serde(with = "my_date_format")]
    pub update_date: NaiveDateTime,
}
