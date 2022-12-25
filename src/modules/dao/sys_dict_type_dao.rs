use std::collections::HashMap;

use mysql::prelude::Queryable;

use crate::{
    common::{
        entity::{dict_type::DictType, sys_dict_type_entity::SysDictTypeEntity},
        utils::page_data::{query_order, PageData},
    },
    modules::{dto::sys_dict_type_dto::SysDictTypeDto, get_mysql_conn},
};

pub struct SysDictTypeDao;

impl SysDictTypeDao {
    pub fn get_dict_type_list() -> Vec<DictType> {
        let mut conn = get_mysql_conn();
        let sql = "select id, dict_type from sys_dict_type order by dict_type, sort";
        conn.query_map(sql, |(id, dict_type)| DictType {
            id,
            dict_type,
            ..Default::default()
        })
        .unwrap()
    }

    pub fn page(params: HashMap<String, String>) -> PageData<SysDictTypeDto> {
        let mut conn = get_mysql_conn();
        let order_by = query_order(&params, "sort".to_string(), false);

        let mut where_sql = "where 1 = 1".to_string();

        if let Some(dict_type) = params.get("dictType") {
            where_sql.push_str(&format!(" and t1.dict_type like '%{}%'", dict_type));
        }

        if let Some(dict_name) = params.get("dictName") {
            where_sql.push_str(&format!(" and t1.dict_name like '%{}%'", dict_name));
        }

        let count_sql = format!(
            "select SQL_NO_CACHE count(*) from sys_dict_type t1 {}",
            where_sql
        );

        let select_sql = format!(
            "select SQL_NO_CACHE t1.* from sys_dict_type t1 {} {}",
            where_sql, order_by
        );

        let count = conn
            .query_first::<i64, _>(count_sql)
            .unwrap()
            .unwrap_or_default();

        let list = conn
            .exec::<SysDictTypeEntity, _, _>(select_sql, ())
            .unwrap();

        let list = list.iter().map(SysDictTypeDto::from).collect();

        PageData::new(count, list)
    }

    pub fn insert(entity: SysDictTypeEntity) {
        let mut conn = get_mysql_conn();
        let sql = "insert into sys_dict_type (dict_type, dict_name, remark, sort, creator, create_date, updater, update_date) values (?, ?, ?, ?, ?, ?, ?, ?)";
        conn.exec_drop(
            sql,
            (
                entity.dict_type,
                entity.dict_name,
                entity.remark,
                entity.sort,
                entity.creator,
                entity.create_date.format("%Y-%m-%d %H:%M:%S").to_string(),
                entity.updater,
                entity.update_date.format("%Y-%m-%d %H:%M:%S").to_string(),
            ),
        )
        .unwrap();
    }

    pub fn get(id: i64) -> SysDictTypeEntity {
        let mut conn = get_mysql_conn();
        let sql = "select * from sys_dict_type where id = ?";
        conn.exec_first::<SysDictTypeEntity, _, _>(sql, (id,))
            .unwrap()
            .unwrap()
    }

    pub fn update_by_id(entity: SysDictTypeEntity) {
        let mut conn = get_mysql_conn();
        let sql = "update sys_dict_type set dict_type = ?, dict_name = ?, remark = ?, sort = ?, updater = ?, update_date = ? where id = ?";
        conn.exec_drop(
            sql,
            (
                entity.dict_type,
                entity.dict_name,
                entity.remark,
                entity.sort,
                entity.updater,
                entity.update_date.format("%Y-%m-%d %H:%M:%S").to_string(),
                entity.id,
            ),
        )
        .unwrap();
    }

    pub fn delete_batch_ids(ids: Vec<i64>) {
        let mut conn = get_mysql_conn();
        let ids = ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let sql = "delete from sys_dict_type where id in (?)";
        conn.exec_drop(sql, (ids,)).unwrap();
    }
}
