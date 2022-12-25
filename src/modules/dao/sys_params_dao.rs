use std::collections::HashMap;

use mysql::{params, prelude::Queryable};

use crate::{
    common::{
        entity::sys_params_entity::SysParamsEntity,
        utils::page_data::{query_order, PageData},
    },
    modules::{dto::sys_params_dto::SysParamsDto, get_mysql_conn},
};

pub struct SysParamsDao;
impl SysParamsDao {
    pub fn page(params: HashMap<String, String>) -> PageData<SysParamsDto> {
        let mut conn = get_mysql_conn();
        let order_by = query_order(&params, "create_date".to_string(), false);

        let mut where_sql = "where param_type = 1".to_string();

        if let Some(param_code) = params.get("paramCode") {
            where_sql.push_str(&format!(" and param_code like '%{}%'", param_code));
        }

        let count_sql = format!("select SQL_NO_CACHE count(*) from sys_params {}", where_sql);

        let select_sql = format!(
            "select SQL_NO_CACHE * from sys_params {} {}",
            where_sql, order_by
        );

        let count = conn
            .query_first::<i64, _>(count_sql)
            .unwrap()
            .unwrap_or_default();

        let list = conn.exec::<SysParamsEntity, _, _>(select_sql, ()).unwrap();

        let list = list
            .into_iter()
            .map(|entity| SysParamsDto::from(&entity))
            .collect();

        PageData::new(count, list)
    }

    pub fn delete_batch_ids(ids: Vec<i64>) {
        let mut conn = get_mysql_conn();
        let ids = ids
            .into_iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let sql = "delete from sys_params where id in (:ids)";
        conn.exec_drop(sql, params! {"ids" => ids}).unwrap();
    }

    pub fn get_param_code_list(ids: Vec<i64>) -> Vec<String> {
        let mut conn = get_mysql_conn();
        let ids = ids
            .into_iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let sql = "select param_code from sys_params where id in (:ids)";
        conn.exec::<String, _, _>(sql, params! {"ids" => ids})
            .unwrap()
    }

    pub fn insert(entity: SysParamsEntity) {
        let mut conn = get_mysql_conn();
        let sql = "insert into sys_params (param_code, param_value, remark, param_type, creator, updater, create_date, update_date) values (:param_code, :param_value, :remark, :param_type, :creator, :updater, :create_date, :update_date)";
        conn.exec_drop(
            sql,
            params! {
                "param_code" => entity.param_code,
                "param_value" => entity.param_value,
                "remark" => entity.remark,
                "param_type" => entity.param_type,
                "creator" => entity.creator,
                "updater" => entity.updater,
                "create_date" => entity.create_date.format("%Y-%m-%d %H:%M:%S").to_string(),
                "update_date" => entity.update_date.format("%Y-%m-%d %H:%M:%S").to_string(),
            },
        )
        .unwrap();
    }

    pub fn select_by_id(id: i64) -> SysParamsEntity {
        let mut conn = get_mysql_conn();
        let sql = "select * from sys_params where id = :id";
        conn.exec_first::<SysParamsEntity, _, _>(sql, params! {"id" => id})
            .unwrap()
            .unwrap()
    }

    pub fn update_by_id(entity: SysParamsEntity) {
        let mut conn = get_mysql_conn();
        let sql = "update sys_params set param_code = :param_code, param_value = :param_value, remark = :remark, updater = :updater, update_date = :update_date where id = :id";
        conn.exec_drop(
            sql,
            params! {
                "param_code" => entity.param_code,
                "param_value" => entity.param_value,
                "remark" => entity.remark,
                "updater" => entity.updater,
                "update_date" => entity.update_date.format("%Y-%m-%d %H:%M:%S").to_string(),
                "id" => entity.id,
            },
        )
        .unwrap();
    }
}
