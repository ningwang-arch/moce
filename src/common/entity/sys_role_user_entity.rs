use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct SysRoleUserEntity {
    pub id: i64,
    pub role_id: i64,
    pub user_id: i64,
    pub creator: i64,
    pub create_time: NaiveDateTime,
}
