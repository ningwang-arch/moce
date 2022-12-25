use std::collections::HashMap;

use mysql::{params, prelude::Queryable};

use crate::{
    common::{
        entity::sys_role_entity::SysRoleEntity,
        utils::page_data::{query_order, PageData},
    },
    modules::{dto::sys_role_dto::SysRoleDto, get_mysql_conn},
};

pub struct SysRoleDao;

impl SysRoleDao {
    pub fn page(params: &HashMap<String, String>) -> PageData<SysRoleDto> {
        let mut conn = get_mysql_conn();
        let order_by = query_order(params, "create_date".to_string(), false);

        let mut where_sql = "where 1 = 1".to_string();

        if let Some(name) = params.get("name") {
            where_sql.push_str(&format!(" and t1.name like '%{}%'", name));
        }

        if let Some(dept_id_list) = params.get("deptIdList") {
            if !dept_id_list.trim().is_empty() {
                where_sql.push_str(&format!(" and t1.dept_id in ({})", dept_id_list));
            }
        }

        let count_sql = format!(
            "select SQL_NO_CACHE count(*) from sys_role t1 {}",
            where_sql
        );

        let select_sql = format!(
            "select SQL_NO_CACHE t1.* from sys_role t1 {} {}",
            where_sql, order_by
        );

        let count = conn
            .query_first::<i64, _>(count_sql)
            .unwrap()
            .unwrap_or_default();

        let list = conn.exec::<SysRoleEntity, _, _>(select_sql, ()).unwrap();

        let list = list.iter().map(SysRoleDto::from).collect();

        PageData::new(count, list)
    }

    pub fn insert(entity: SysRoleEntity) -> i64 {
        let mut conn = get_mysql_conn();
        conn.exec_drop(
            "insert into sys_role (name, remark, dept_id, creator, create_date, updater, update_date) values (:name, :remark, :dept_id, :creator, :create_date, :updater, :update_date)",
            params! {
                "name" => entity.name,
                "remark" => entity.remark,
                "dept_id" => entity.dept_id,
                "creator" => entity.creator,
                "create_date" => entity.create_date.format("%Y-%m-%d %H:%M:%S").to_string(),
                "updater" => entity.updater,
                "update_date" => entity.update_date.format("%Y-%m-%d %H:%M:%S").to_string(),
            },
        ).unwrap();

        conn.last_insert_id() as i64
    }

    pub fn select_by_id(id: i64) -> SysRoleEntity {
        let mut conn = get_mysql_conn();
        conn.exec_first::<SysRoleEntity, _, _>(
            "select * from sys_role where id = :id",
            params! {
                "id" => id,
            },
        )
        .unwrap()
        .unwrap()
    }

    pub fn select_list(map: &HashMap<String, String>) -> Vec<SysRoleEntity> {
        let mut conn = get_mysql_conn();
        let mut where_sql = "where 1 = 1".to_string();

        if let Some(name) = map.get("name") {
            where_sql.push_str(&format!(" and name like '%{}%'", name));
        }

        if let Some(dept_id_list) = map.get("deptIdList") {
            if !dept_id_list.trim().is_empty() {
                where_sql.push_str(&format!(" and dept_id in ({})", dept_id_list));
            }
        }

        let sql = format!(
            "select * from sys_role {} order by create_date desc",
            where_sql
        );

        conn.exec::<SysRoleEntity, _, _>(sql, ()).unwrap()
    }

    pub fn update_by_id(entity: SysRoleEntity) {
        let mut conn = get_mysql_conn();
        conn.exec_drop(
            "update sys_role set name = :name, remark = :remark, dept_id = :dept_id, updater = :updater, update_date = :update_date where id = :id",
            params! {
                "name" => entity.name,
                "remark" => entity.remark,
                "dept_id" => entity.dept_id,
                "updater" => entity.updater,
                "update_date" => entity.update_date.format("%Y-%m-%d %H:%M:%S").to_string(),
                "id" => entity.id,
            },
        ).unwrap();
    }

    pub fn delete_batch_ids(ids: Vec<i64>) {
        let mut conn = get_mysql_conn();
        conn.exec_drop(
            "delete from sys_role where id in (:ids)",
            params! {
                "ids" => ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(",")
            },
        )
        .unwrap();
    }
}
