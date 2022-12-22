use chrono::NaiveDateTime;
use rocket::{
    request::{FromRequest, Outcome},
    Request,
};

#[derive(Debug, Default)]
pub struct SysLogLoginEntity {
    pub id: i64,
    pub operation: i64, // 0:登录 1:登出
    pub status: i64,    // 0:失败 1:成功 2:账号已锁定
    pub user_agent: String,
    pub ip: String,
    pub creator: i64,
    pub creator_name: String,
    pub create_time: NaiveDateTime,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SysLogLoginEntity {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let entity = SysLogLoginEntity {
            user_agent: req
                .headers()
                .get_one("User-Agent")
                .unwrap_or("")
                .to_string(),
            ip: get_ipaddr(req),
            create_time: chrono::Local::now().naive_local(),
            ..Default::default()
        };
        Outcome::Success(entity)
    }
}

fn get_ipaddr(req: &Request<'_>) -> String {
    let mut ip = String::new();
    let unknown = String::from("unknown");
    if let Some(x_forwarded_for) = req.headers().get_one("x-forwarded-for") {
        if !x_forwarded_for.is_empty() && x_forwarded_for != unknown {
            ip = x_forwarded_for.to_string();
        }
    }
    if ip.is_empty() {
        if let Some(proxy_client_ip) = req.headers().get_one("Proxy-Client-IP") {
            if !proxy_client_ip.is_empty() && proxy_client_ip != unknown {
                ip = proxy_client_ip.to_string();
            }
        }
    }
    if ip.is_empty() {
        if let Some(wl_proxy_client_ip) = req.headers().get_one("WL-Proxy-Client-IP") {
            if !wl_proxy_client_ip.is_empty() && wl_proxy_client_ip != unknown {
                ip = wl_proxy_client_ip.to_string();
            }
        }
    }
    if ip.is_empty() {
        if let Some(http_client_ip) = req.headers().get_one("HTTP_CLIENT_IP") {
            if !http_client_ip.is_empty() && http_client_ip != unknown {
                ip = http_client_ip.to_string();
            }
        }
    }
    if ip.is_empty() {
        if let Some(http_x_forwarded_for) = req.headers().get_one("HTTP_X_FORWARDED_FOR") {
            if !http_x_forwarded_for.is_empty() && http_x_forwarded_for != unknown {
                ip = http_x_forwarded_for.to_string();
            }
        }
    }
    if ip.is_empty() {
        if let Some(remote_addr) = req.remote() {
            ip = remote_addr.ip().to_string();
        }
    }
    ip
}
