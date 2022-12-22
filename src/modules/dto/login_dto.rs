use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginDto {
    pub username: String,
    pub password: String,
    pub captcha: String,
    pub uuid: String,
}
