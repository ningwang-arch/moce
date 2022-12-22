use mysql::prelude::Queryable;

use crate::{common::entity::dict_data::DictData, modules::get_mysql_conn};

pub struct SysDictDataDao;

impl SysDictDataDao {
    pub fn get_dict_data_list() -> Vec<DictData> {
        let mut conn = get_mysql_conn();
        let sql = "select dict_type_id, dict_label, dict_value from sys_dict_data order by dict_type_id, sort";
        conn.query_map(sql, |(dict_type_id, dict_label, dict_value)| DictData {
            dict_type_id,
            dict_label,
            dict_value,
        })
        .unwrap()
    }
}
