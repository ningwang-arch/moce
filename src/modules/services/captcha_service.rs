use captcha_rs::CaptchaBuilder;

use crate::common::redis::{redis_keys::get_captcha_key, redis_utils::RedisUtils};

pub struct CaptchaService {
    redis_utils: RedisUtils,
}

impl CaptchaService {
    pub fn new() -> Self {
        CaptchaService {
            redis_utils: RedisUtils::new(),
        }
    }

    pub fn create(&mut self, uuid: String) -> Vec<u8> {
        let captcha = CaptchaBuilder::new()
            .length(5)
            .width(150)
            .height(40)
            .complexity(4) // min: 1, max: 10
            .build();

        let code = captcha.text.clone();
        let img = base64::decode(
            captcha
                .base_img
                .strip_prefix("data:image/png;base64,")
                .unwrap(),
        );

        self.set_cache(uuid, code);

        img.unwrap()
    }

    pub fn verify(&mut self, uuid: String, code: String) -> bool {
        let cache_code = self.get_cache(uuid);

        if cache_code.is_empty() {
            return false;
        }

        if code.to_lowercase() != cache_code.to_lowercase() {
            return false;
        }

        true
    }

    fn set_cache(&mut self, uuid: String, code: String) {
        let key = get_captcha_key(uuid);
        self.redis_utils.set(&key, &code, 300).unwrap();
    }

    fn get_cache(&mut self, uuid: String) -> String {
        let key = get_captcha_key(uuid);
        match self.redis_utils.get(&key, 300) {
            Ok(value) => {
                self.redis_utils.delete(&key).unwrap();
                value
            }
            Err(_) => "".to_string(),
        }
    }
}

impl Default for CaptchaService {
    fn default() -> Self {
        Self::new()
    }
}
