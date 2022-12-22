use mysql::{Opts, Pool};
use rocket::fairing::AdHoc;
use serde::{Deserialize, Serialize};

use crate::CONFIG;

pub mod controller;
pub mod dao;
pub mod dto;
pub mod services;

pub enum ErrorCode {
    InternalServerError = 500,
    UNAUTHORIZED = 401,
    NotNull = 10001,
    DbRecordExists = 10002,
    ParamsGetError = 10003,
    AccountPasswordError = 10004,
    AccountDisable = 10005,
    IdentifierNotNull = 10006,
    CaptchaError = 10007,
    SubMenuExist = 10008,
    PasswordError = 10009,
    SuperiorDeptError = 10011,
    SuperiorMenuError = 10012,
    DataScopeParamsError = 10013,
    DeptSubDeleteError = 10014,
    DeptUserDeleteError = 10015,
    UploadFileEmpty = 10019,
    TokenNotEmpty = 10020,
    TokenInvalid = 10021,
    AccountLock = 10022,
    OssUploadFileError = 10024,
    RedisError = 10027,
    JobError = 10028,
    InvalidSymbol = 10029,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseWrapper {
    pub code: i32,
    pub msg: String,
    pub data: Option<serde_json::Value>,
}

impl ResponseWrapper {
    pub fn new(code: i32, msg: String, data: Option<serde_json::Value>) -> Self {
        ResponseWrapper { code, msg, data }
    }
}

struct MysqlConn {
    pool: Pool,
}

impl MysqlConn {
    fn new() -> Self {
        let config = &CONFIG.mysql;

        let database_url = format!(
            "mysql://{}:{}@{}:{}/{}",
            config.user, config.password, config.host, config.port, config.database
        );

        let pool = Pool::new(Opts::from_url(&database_url).unwrap()).unwrap();

        MysqlConn { pool }
    }

    fn get_conn(&self) -> mysql::PooledConn {
        self.pool.get_conn().unwrap()
    }
}

lazy_static::lazy_static! {
    static ref MYSQL_CONN: MysqlConn = MysqlConn::new();
}

pub fn get_mysql_conn() -> mysql::PooledConn {
    MYSQL_CONN.get_conn()
}

mod my_date_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let date = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(date)
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Main", |rocket| async {
        rocket.attach(controller::stage())
    })
}
