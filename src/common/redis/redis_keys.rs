pub fn get_params_key() -> String {
    "sys:params".to_string()
}

pub fn get_captcha_key(uuid: String) -> String {
    format!("sys:captcha:{}", uuid)
}

pub fn get_user_permissions_key(user_id: i64) -> String {
    format!("sys:user:permissions:{}", user_id)
}
