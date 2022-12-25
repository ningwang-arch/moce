use crate::common::redis::{redis_keys::get_params_key, redis_utils::RedisUtils};

#[derive(Default)]
pub struct SysParamsRedis {
    redis: RedisUtils,
}

const DEFAULT_EXPIRE: i64 = 60 * 60 * 24;

impl SysParamsRedis {
    pub fn new() -> Self {
        Self {
            redis: RedisUtils::new(),
        }
    }

    pub fn delete(&mut self, param_codes: Vec<String>) {
        let param_codes = param_codes
            .iter()
            .map(|param_code| param_code.as_str())
            .collect::<Vec<&str>>();
        let key = get_params_key();
        self.redis.h_del(&key, param_codes).unwrap();
    }

    pub fn set(&mut self, param_code: &str, param_value: &str) {
        if param_value.is_empty() {
            return;
        }

        let key = get_params_key();
        self.redis
            .h_set(&key, param_code, param_value, DEFAULT_EXPIRE)
            .unwrap();
    }

    pub fn get(&mut self, param_code: &str) -> Option<String> {
        let key = get_params_key();
        let ret = self.redis.h_get(&key, param_code);
        if ret.is_err() {
            return None;
        }
        Some(ret.unwrap())
    }
}
