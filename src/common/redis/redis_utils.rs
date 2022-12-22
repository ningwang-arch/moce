use super::{get_redis_conn, NOT_EXPIRE};

pub struct RedisUtils {
    conn: redis::Connection,
}

impl Default for RedisUtils {
    fn default() -> Self {
        Self::new()
    }
}

impl RedisUtils {
    pub fn new() -> Self {
        RedisUtils {
            conn: get_redis_conn(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str, expire: i64) -> Result<(), redis::RedisError> {
        redis::cmd("SET")
            .arg(key)
            .arg(value)
            .query(&mut self.conn)?;

        if expire != NOT_EXPIRE {
            self.expire(key, expire)?;
        }

        Ok(())
    }

    pub fn get(&mut self, key: &str, expire: i64) -> Result<String, redis::RedisError> {
        let value: String = redis::cmd("GET").arg(key).query(&mut self.conn)?;

        if expire != NOT_EXPIRE {
            self.expire(key, expire)?;
        }

        Ok(value)
    }

    pub fn delete(&mut self, key: &str) -> Result<(), redis::RedisError> {
        redis::cmd("DEL").arg(key).query(&mut self.conn)?;

        Ok(())
    }

    pub fn delete_keys(&mut self, keys: Vec<&str>) -> Result<(), redis::RedisError> {
        redis::cmd("DEL").arg(keys).query(&mut self.conn)?;

        Ok(())
    }

    pub fn h_set(
        &mut self,
        key: &str,
        field: &str,
        value: &str,
        expire: i64,
    ) -> Result<(), redis::RedisError> {
        redis::cmd("HSET")
            .arg(key)
            .arg(field)
            .arg(value)
            .query(&mut self.conn)?;

        if expire != NOT_EXPIRE {
            self.expire(key, expire)?;
        }

        Ok(())
    }

    pub fn h_get(&mut self, key: &str, field: &str) -> Result<String, redis::RedisError> {
        let value: String = redis::cmd("HGET")
            .arg(key)
            .arg(field)
            .query(&mut self.conn)?;

        Ok(value)
    }

    pub fn h_del(&mut self, key: &str, fields: Vec<&str>) -> Result<(), redis::RedisError> {
        redis::cmd("HDEL")
            .arg(key)
            .arg(fields)
            .query(&mut self.conn)?;

        Ok(())
    }

    pub fn expire(&mut self, key: &str, expire: i64) -> Result<(), redis::RedisError> {
        redis::cmd("EXPIRE")
            .arg(key)
            .arg(expire)
            .query(&mut self.conn)?;
        Ok(())
    }
}
