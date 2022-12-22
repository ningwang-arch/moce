use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysRoleMenuEntity {
    pub id: i64,
    pub role_id: i64,
    pub menu_id: i64,
    pub creator: i64,
    pub create_date: NaiveDateTime,
}
