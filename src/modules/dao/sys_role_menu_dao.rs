use mysql::{params, prelude::Queryable};

use crate::{common::entity::sys_role_menu_entity::SysRoleMenuEntity, modules::get_mysql_conn};

pub struct SysRoleMenuDao;

impl SysRoleMenuDao {
    pub fn delete_by_role_ids(role_ids: Vec<i64>) {
        let role_ids_str = role_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let mut conn = get_mysql_conn();
        conn.exec_drop(
            "delete from sys_role_menu where role_id in (:role_ids)",
            params! {
                "role_ids" => role_ids_str,
            },
        )
        .unwrap();
    }

    pub fn insert(entity: SysRoleMenuEntity) {
        let mut conn = get_mysql_conn();
        conn.exec_drop(
            "insert into sys_role_menu (role_id, menu_id, creator, create_date) values (:role_id, :menu_id, :creator, :create_date)",
            params! {
                "role_id" => entity.role_id,
                "menu_id" => entity.menu_id,
                "creator" => entity.creator,
                "create_date" => entity.create_date.format("%Y-%m-%d %H:%M:%S").to_string(),
            },
        )
        .unwrap();
    }

    pub fn get_menu_id_list(id: i64) -> Vec<i64> {
        let mut conn = get_mysql_conn();
        let list = conn
            .exec::<(i64,), _, _>(
                "select menu_id from sys_role_menu where role_id = :id",
                params! {
                    "id" => id,
                },
            )
            .unwrap();
        list.iter().map(|(menu_id,)| *menu_id).collect()
    }

    pub fn delete_by_menu_id(id: i64) {
        let mut conn = get_mysql_conn();
        conn.exec_drop(
            "delete from sys_role_menu where menu_id = :id",
            params! {
                "id" => id,
            },
        )
        .unwrap();
    }
}
