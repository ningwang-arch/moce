use mysql::{params, prelude::Queryable};

use crate::{common::entity::sys_role_user_entity::SysRoleUserEntity, modules::get_mysql_conn};

pub struct SysRoleUserDao;

impl SysRoleUserDao {
    pub fn get_role_id_list(id: i64) -> Vec<i64> {
        let mut conn = get_mysql_conn();
        let sql = "select role_id from sys_role_user where user_id = :id";

        let list = conn
            .exec::<(i64,), _, _>(sql, params! {"id" => id,})
            .unwrap();
        list.iter().map(|(role_id,)| *role_id).collect()
    }

    pub fn delete_by_user_ids(vec: Vec<i64>) {
        let mut conn = get_mysql_conn();
        let ids: String = vec
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let sql = "delete from sys_role_user where user_id in (:ids)";

        conn.exec::<usize, _, _>(sql, params! {"ids" => ids,})
            .unwrap();
    }

    pub fn insert(entity: SysRoleUserEntity) {
        let mut conn = get_mysql_conn();
        let sql = "insert into sys_role_user (role_id, user_id, creator, create_date) values (:role_id, :user_id, :creator, :create_time)";

        conn.exec::<usize, _, _>(
            sql,
            params! {
                "role_id" => entity.role_id,
                "user_id" => entity.user_id,
                "creator" => entity.creator,
                "create_time" => entity.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            },
        )
        .unwrap();
    }

    pub fn delete_by_role_ids(ids: Vec<i64>) {
        let mut conn = get_mysql_conn();
        let ids: String = ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let sql = "delete from sys_role_user where role_id in (:ids)";

        conn.exec::<usize, _, _>(sql, params! {"ids" => ids,})
            .unwrap();
    }
}
