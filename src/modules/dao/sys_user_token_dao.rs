use mysql::{params, prelude::Queryable};

use crate::{common::entity::sys_user_token_entity::SysUserTokenEntity, modules::get_mysql_conn};

pub struct SysUserTokenDao;

impl SysUserTokenDao {
    pub fn get_by_userid(user_id: i64) -> Option<SysUserTokenEntity> {
        let mut conn = get_mysql_conn();
        let sql = "select * from sys_user_token where user_id = :user_id";
        let result =
            conn.exec_first::<SysUserTokenEntity, _, _>(sql, params! {"user_id" => user_id});
        match result {
            Ok(Some(user)) => Some(user),
            _ => None,
        }
    }

    pub fn insert(token: SysUserTokenEntity) {
        let mut conn = get_mysql_conn();
        let sql = "INSERT INTO sys_user_token (user_id, token, expire_date, update_date, create_date) VALUES (:user_id, :token, :expire_time, :update_time, :create_time)";
        conn.exec_drop(
            sql,
            params! {
                "user_id" => token.user_id,
                "token" => token.token,
                "expire_time" => token.expire_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                "update_time" => token.update_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                "create_time" => token.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            },
        )
        .unwrap();
    }

    pub fn update_by_id(token: SysUserTokenEntity) {
        let mut conn = get_mysql_conn();

        let sql = "UPDATE sys_user_token SET token = :token, expire_date = :expire_time, update_date = :update_time WHERE id = :id";
        conn.exec_drop(
            sql,
            params! {
                "id" => token.id,
                "token" => token.token,
                "expire_time" => token.expire_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                "update_time" => token.update_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            },
        )
        .unwrap();
    }

    pub fn get_by_token(token: String) -> Option<SysUserTokenEntity> {
        let mut conn = get_mysql_conn();
        let sql = "select * from sys_user_token where token = :token";
        let result = conn.exec_first::<SysUserTokenEntity, _, _>(sql, params! {"token" => token});
        match result {
            Ok(Some(user)) => Some(user),
            _ => None,
        }
    }

    pub fn update_token(token: String, user_id: i64) {
        let mut conn = get_mysql_conn();
        let sql = "UPDATE sys_user_token SET token = :token WHERE user_id = :user_id";
        conn.exec_drop(sql, params! {"token" => token, "user_id" => user_id})
            .unwrap();
    }
}
