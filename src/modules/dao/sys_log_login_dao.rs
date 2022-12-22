use mysql::{params, prelude::Queryable};

use crate::{common::entity::sys_log_login_entity::SysLogLoginEntity, modules::get_mysql_conn};

pub struct SysLogLoginDao;

impl SysLogLoginDao {
    pub fn insert(log: SysLogLoginEntity) {
        let mut conn = get_mysql_conn();
        let sql = "INSERT INTO sys_log_login (operation, status, user_agent, ip,  creator_name, creator, create_date) VALUES (:operation, :status, :user_agent, :ip, :creator_name, :creator, :create_date)";
        conn.exec_drop(
            sql,
            params! {
                "operation" => log.operation,
                "status" => log.status,
                "user_agent" => log.user_agent,
                "ip" => log.ip,
                "creator_name" => log.creator_name,
                "creator" => log.creator,
                "create_date" => log.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            },
        )
        .unwrap();
    }
}
