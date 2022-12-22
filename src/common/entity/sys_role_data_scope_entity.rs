use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysRoleDataScopeEntity {
    pub id: i64,
    pub role_id: i64,
    pub dept_id: i64,
    pub creator: i64,
    pub create_date: NaiveDateTime,
}
