use moce::{
    common::redis::{redis_keys::get_user_permissions_key, redis_utils::RedisUtils},
    modules::{dao::sys_user_token_dao::SysUserTokenDao, stage, ErrorCode, ResponseWrapper},
    CONFIG,
};
use rocket::{
    config::Ident,
    fairing::{Fairing, Info, Kind},
    http::Header,
    serde::json::Json,
    Config, Request, Response,
};
use rocket_grants::GrantsFairing;

#[macro_use]
extern crate rocket;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        let origin = _request
            .headers()
            .get_one("Origin")
            .unwrap_or("")
            .to_string();
        let origin = if origin.is_empty() {
            "*".to_string()
        } else {
            origin
        };
        response.set_header(Header::new("Access-Control-Allow-Origin", origin));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET,POST,PUT,DELETE,OPTIONS",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Content-Type,Authorization,Accept,X-Requested-With,X-File-Name,X-File-Size,X-File-Type,X-File-Ext,Token",
        ));
        response.set_header(Header::new("Access-Control-Max-Age", "1728000"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        response.set_header(Header::new(
            "Access-Control-Expose-Headers",
            "Content-Disposition",
        ));
    }
}

#[options("/<_..>")]
fn all_options() -> String {
    String::from("ok")
}

async fn extract(req: &mut rocket::Request<'_>) -> Option<Vec<String>> {
    let mut token = req.headers().get_one("token").unwrap_or("").to_string();
    if token.is_empty() {
        if let Some(cookie) = req.cookies().get("token") {
            token = cookie.value().to_string();
        } else {
            return None;
        }
    }

    let token_entity = SysUserTokenDao::get_by_token(token);
    token_entity.as_ref()?;

    let token_entity = token_entity.unwrap();

    let key = get_user_permissions_key(token_entity.user_id);
    let mut redis_conn = RedisUtils::new();
    let perms_str = redis_conn.get(&key, 3600);
    if perms_str.is_err() {
        return None;
    }
    let perms: Vec<String> = perms_str
        .unwrap()
        .split(',')
        .map(|s| s.to_string())
        .collect();
    Some(perms)
}

#[catch(401)]
fn unauthorized() -> Json<ResponseWrapper> {
    Json(ResponseWrapper::new(
        ErrorCode::TokenInvalid as i32,
        "unauthorized".to_string(),
        None,
    ))
}

#[catch(403)]
fn forbidden() -> Json<ResponseWrapper> {
    Json(ResponseWrapper::new(
        ErrorCode::UNAUTHORIZED as i32,
        "forbidden".to_string(),
        None,
    ))
}

#[launch]
fn rocket() -> _ {
    let rocket_config = &CONFIG.rocket;

    let config = Config {
        address: rocket_config.address.parse().unwrap(),
        port: rocket_config.port,
        workers: rocket_config.workers,
        log_level: rocket_config.log_level.parse().unwrap(),
        ident: Ident::try_new(rocket_config.ident.clone()).unwrap(),

        ..Config::default()
    };

    rocket::build()
        .configure(config)
        .attach(CORS)
        .register("/", catchers![unauthorized, forbidden])
        .mount("/", routes![all_options])
        .attach(stage())
        .attach(GrantsFairing::with_extractor_fn(|req| {
            Box::pin(extract(req)) // example with a separate async function, but you can write a closure right here
        }))
}
