use chrono::NaiveDateTime;
use serde_json::Map;

use crate::{
    common::entity::sys_user_token_entity::SysUserTokenEntity,
    modules::{dao::sys_user_token_dao::SysUserTokenDao, ResponseWrapper},
    TOKEN_HEADER,
};

pub struct SysUserTokenService;

const EXPIRE: i64 = 3600 * 12;

impl SysUserTokenService {
    pub fn create_token(user_id: i64) -> ResponseWrapper {
        let now: NaiveDateTime = chrono::Local::now().naive_local();
        let expire = now + chrono::Duration::seconds(EXPIRE);

        let token_entity = SysUserTokenDao::get_by_userid(user_id);

        let token;
        match token_entity {
            Some(mut entity) => {
                if entity.expire_time < now {
                    token = Self::generate_value(uuid::Uuid::new_v4().to_string());
                } else {
                    token = entity.token;
                }
                entity.token = token.clone();
                entity.expire_time = expire;
                entity.update_time = now;
                SysUserTokenDao::update_by_id(entity);
            }
            None => {
                token = Self::generate_value(uuid::Uuid::new_v4().to_string());
                let entity = SysUserTokenEntity {
                    id: 0,
                    user_id,
                    token: token.clone(),
                    expire_time: expire,
                    update_time: now,
                    create_time: now,
                };
                SysUserTokenDao::insert(entity);
            }
        }
        let mut map = Map::new();
        map.insert(TOKEN_HEADER.to_string(), serde_json::Value::String(token));
        map.insert(
            "expire".to_string(),
            serde_json::Value::Number(serde_json::Number::from(EXPIRE)),
        );

        ResponseWrapper {
            code: 0,
            msg: "success".to_string(),
            data: Some(serde_json::Value::Object(map)),
        }
    }

    pub fn logout(user_id: i64) {
        let token = Self::generate_value(uuid::Uuid::new_v4().to_string());
        SysUserTokenDao::update_token(token, user_id);
    }

    fn generate_value(param: String) -> String {
        let data = md5::compute(param);
        format!("{:x}", data)
    }
}
