use mysql::{params, prelude::Queryable};

use crate::{
    common::entity::sys_role_data_scope_entity::SysRoleDataScopeEntity, modules::get_mysql_conn,
};

pub struct SysRoleDataScopeDao;

impl SysRoleDataScopeDao {
    pub fn delete_by_role_ids(role_ids: Vec<i64>) {
        let mut conn = get_mysql_conn();
        let role_ids = role_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");
        conn.exec_drop(
            "delete from sys_role_data_scope where role_id in (:role_ids)",
            params! {
                "role_ids" => role_ids,
            },
        )
        .unwrap();
    }

    pub fn insert(entity: SysRoleDataScopeEntity) {
        let mut conn = get_mysql_conn();
        conn.exec_drop(
            "insert into sys_role_data_scope (role_id, dept_id, creator, create_date) values (:role_id, :dept_id, :creator, :create_date)",
            params! {
                "role_id" => entity.role_id,
                "dept_id" => entity.dept_id,
                "creator" => entity.creator,
                "create_date" => entity.create_date.format("%Y-%m-%d %H:%M:%S").to_string(),
            },
        )
        .unwrap();
    }

    pub fn get_dept_id_list(id: i64) -> Vec<i64> {
        let mut conn = get_mysql_conn();
        let list = conn
            .exec::<(i64,), _, _>(
                "select dept_id from sys_role_data_scope where role_id = :id",
                params! {
                    "id" => id,
                },
            )
            .unwrap();
        list.iter().map(|(dept_id,)| *dept_id).collect()
    }
}
