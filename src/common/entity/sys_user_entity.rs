use chrono::{NaiveDate, NaiveDateTime};
use mysql::{prelude::FromRow, Value};
use rocket::{
    request::{FromRequest, Outcome},
    serde::json::Json,
    Request,
};
use serde::{Deserialize, Serialize};

use crate::modules::{
    dao::{sys_user_dao::SysUserDao, sys_user_token_dao::SysUserTokenDao},
    dto::sys_user_dto::SysUserDto,
    ErrorCode, ResponseWrapper,
};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SysUserEntity {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub real_name: String,
    pub head_url: Option<String>,
    pub gender: i32,
    pub email: String,
    pub mobile: String,
    pub dept_id: Option<i64>,
    pub super_admin: i32,
    pub status: i32,
    pub creator: i64,
    pub create_time: NaiveDateTime,
    pub updater: i64,
    pub update_time: NaiveDateTime,
    pub dept_name: Option<String>,
}

impl SysUserEntity {
    pub fn from(dto: &SysUserDto) -> Self {
        SysUserEntity {
            id: dto.id,
            username: dto.username.clone(),
            password: dto.password.clone(),
            real_name: dto.real_name.clone(),
            head_url: dto.head_url.clone(),
            gender: dto.gender,
            email: dto.email.clone(),
            mobile: dto.mobile.clone(),
            dept_id: dto.dept_id,
            super_admin: dto.super_admin,
            status: dto.status,
            creator: 0,
            create_time: dto.create_date,
            updater: 0,
            update_time: dto.create_date,
            dept_name: dto.dept_name.clone(),
        }
    }
}

// Trait to convert Row into a tuple of FromValue implementors up to arity 12

impl FromRow for SysUserEntity {
    fn from_row_opt(mut row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        Ok(SysUserEntity {
            id: row.take("id").unwrap(),
            username: row.take("username").unwrap(),
            password: row.take("password").unwrap(),
            real_name: row.take("real_name").unwrap(),
            head_url: row.take("head_url").unwrap(),
            gender: row.take("gender").unwrap(),
            email: row.take("email").unwrap(),
            mobile: row.take("mobile").unwrap(),
            dept_id: row.take("dept_id").unwrap(),
            super_admin: row.take("super_admin").unwrap(),
            status: row.take("status").unwrap(),
            creator: row.take("creator").unwrap(),
            create_time: convert(row.take("create_date").unwrap()),
            updater: row.take("updater").unwrap(),
            update_time: convert(row.take("update_date").unwrap()),
            dept_name: row.take("dept_name").unwrap(),
        })
    }
}

fn convert(value: Value) -> NaiveDateTime {
    match value {
        Value::Date(y, m, d, h, i, s, us) => NaiveDate::from_ymd_opt(y as i32, m as u32, d as u32)
            .unwrap()
            .and_hms_micro_opt(h as u32, i as u32, s as u32, us),
        _ => NaiveDateTime::from_timestamp_opt(0, 0),
    }
    .unwrap()
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SysUserEntity {
    type Error = Json<ResponseWrapper>;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let mut token = req.headers().get_one("token").unwrap_or("").to_string();
        if token.is_empty() {
            if let Some(cookie) = req.cookies().get("token") {
                token = cookie.value().to_string();
            } else {
                return Outcome::Failure((
                    rocket::http::Status::Ok,
                    Json(ResponseWrapper::new(
                        ErrorCode::UNAUTHORIZED as i32,
                        "token is empty".to_string(),
                        None,
                    )),
                ));
            }
        }

        let token_entity = SysUserTokenDao::get_by_token(token);
        if token_entity.is_none() {
            return Outcome::Failure((
                rocket::http::Status::Ok,
                Json(ResponseWrapper::new(
                    ErrorCode::TokenInvalid as i32,
                    "token is invalid".to_string(),
                    None,
                )),
            ));
        }

        let token_entity = token_entity.unwrap();
        let user = SysUserDao::get_by_user_id(token_entity.user_id);
        if user.is_none() {
            return Outcome::Failure((
                rocket::http::Status::Ok,
                Json(ResponseWrapper::new(
                    ErrorCode::UNAUTHORIZED as i32,
                    "user not exist".to_string(),
                    None,
                )),
            ));
        }

        Outcome::Success(user.unwrap())
    }
}
