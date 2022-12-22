use mysql::prelude::Queryable;

use crate::{common::entity::dict_type::DictType, modules::get_mysql_conn};

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
}
