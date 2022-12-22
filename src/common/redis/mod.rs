use crate::CONFIG;

pub mod redis_keys;
pub mod redis_utils;

// const DEFAULT_EXPIRE: i64 = 60 * 60 * 24;
const NOT_EXPIRE: i64 = -1;

struct RedisConn {
    client: redis::Client,
}

impl RedisConn {
    fn new() -> Self {
        let config = CONFIG.redis.clone();
        let client = redis::Client::open(format!(
            "redis://:{}@{}:{}/{}",
            config.password, config.host, config.port, config.database
        ));
        match client {
            Ok(client) => RedisConn { client },
            Err(e) => {
                panic!("redis client open error: {}", e);
            }
        }
    }

    fn get_conn(&self) -> redis::Connection {
        let conn = self.client.get_connection();
        match conn {
            Ok(conn) => conn,
            Err(e) => {
                panic!("redis get connection error: {}", e);
            }
        }
    }
}

lazy_static::lazy_static! {
    static ref REDIS_CONN: RedisConn = RedisConn::new();
}

pub fn get_redis_conn() -> redis::Connection {
    REDIS_CONN.get_conn()
}
