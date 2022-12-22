use chrono::NaiveDateTime;

#[derive(Debug, Default)]
pub struct SysParamsEntity {
    pub id: i64,
    pub param_code: String,
    pub param_vale: String,
    pub param_type: i32,
    pub remark: String,
    pub creator: i64,
    pub create_date: NaiveDateTime,
    pub updater: i64,
    pub update_date: NaiveDateTime,
}
