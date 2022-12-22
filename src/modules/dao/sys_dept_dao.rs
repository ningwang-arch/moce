use std::collections::HashMap;

use mysql::{params, prelude::Queryable};

use crate::{common::entity::sys_dept_entity::SysDeptEntity, modules::get_mysql_conn};

pub struct SysDeptDao;

impl SysDeptDao {
    pub fn get_sub_dept_id_list(id: i64) -> Vec<i64> {
        let mut conn = get_mysql_conn();
        conn.exec(
            "select id from sys_dept where pids like :id",
            params! {
                "id" => format!("%{}%", id)
            },
        )
        .unwrap()
    }

    pub fn get_list(map: HashMap<String, String>) -> Vec<SysDeptEntity> {
        let mut conn = get_mysql_conn();
        let mut sql = "select t1.*,(select t2.name from sys_dept t2 where t2.id=t1.pid)parentName from sys_dept t1".to_string();

        if let Some(dept_id_list) = map.get("deptIdList") {
            if !dept_id_list.trim().is_empty() {
                sql.push_str(&format!(" where t1.id in ({})", dept_id_list));
            }
        }

        conn.exec::<SysDeptEntity, _, _>(sql, ()).unwrap()
    }

    pub fn get(id: i64) -> SysDeptEntity {
        let mut conn = get_mysql_conn();
        conn.exec_first::<SysDeptEntity, _, _>(
            "select t1.*,(select t2.name from sys_dept t2 where t2.id=t1.pid)parentName from sys_dept t1 where t1.id=:id",
            params! {
                "id" => id
            },
        )
        .unwrap()
        .unwrap()
    }

    pub fn get_all() -> Vec<SysDeptEntity> {
        let mut conn = get_mysql_conn();
        conn.exec::<SysDeptEntity, _, _>(
            "select t1.*,(select t2.name from sys_dept t2 where t2.id=t1.pid)parentName from sys_dept t1",
            (),
        )
        .unwrap()
    }

    pub fn update_by_id(entity: SysDeptEntity) {
        let mut conn = get_mysql_conn();
        conn.exec_drop(
            "update sys_dept set name=:name,pid=:pid,pids=:pids,sort=:sort, update_date=:update_date, updater=:updater where id=:id",
            params! {
                "id" => entity.id,
                "name" => entity.name,
                "pid" => entity.pid,
                "pids" => entity.pids,
                "sort" => entity.sort,
                "update_date" => entity.update_date.format("%Y-%m-%d %H:%M:%S").to_string(),
                "updater" => entity.updater,

            },
        )
        .unwrap();
    }

    pub fn delete_by_id(id: i64) {
        let mut conn = get_mysql_conn();
        conn.exec_drop(
            "delete from sys_dept where id=:id",
            params! {
                "id" => id
            },
        )
        .unwrap();
    }

    pub fn save(entity: SysDeptEntity) {
        let mut conn = get_mysql_conn();
        conn.exec_drop(
            "insert into sys_dept (name,pid,pids,sort,create_date,creator,update_date,updater) values (:name,:pid,:pids,:sort,:create_date,:creator,:update_date,:updater)",
            params! {
                "name" => entity.name,
                "pid" => entity.pid,
                "pids" => entity.pids,
                "sort" => entity.sort,
                "create_date" => entity.create_date.format("%Y-%m-%d %H:%M:%S").to_string(),
                "creator" => entity.creator,
                "update_date" => entity.update_date.format("%Y-%m-%d %H:%M:%S").to_string(),
                "updater" => entity.updater,
            },
        )
        .unwrap();
    }
}
