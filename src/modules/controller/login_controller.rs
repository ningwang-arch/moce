use rocket::fairing::AdHoc;
use rocket::http::ContentType;

use rocket::serde::json::Json;
use rocket::{get, post, routes, Responder};

use crate::common::entity::sys_log_login_entity::SysLogLoginEntity;
use crate::common::entity::sys_user_entity::SysUserEntity;
use crate::modules::dto::login_dto::LoginDto;
use crate::modules::services::captcha_service::CaptchaService;
use crate::modules::services::sys_log_login_service::SysLogLoginService;
use crate::modules::services::sys_user_service::SysUserService;
use crate::modules::services::sys_user_token_service::SysUserTokenService;
use crate::modules::{ErrorCode, ResponseWrapper};
use pwhash::bcrypt;

#[derive(Responder)]
#[allow(clippy::large_enum_variant)]
enum ImageResponse {
    Ok(Vec<u8>, ContentType),
    Fail(Json<ResponseWrapper>),
}

#[get("/captcha?<uuid>")]
fn captcha(uuid: String) -> ImageResponse {
    if uuid.trim().is_empty() {
        return ImageResponse::Fail(Json(ResponseWrapper {
            code: ErrorCode::IdentifierNotNull as i32,
            msg: "Invalid UUID".to_string(),
            data: None,
        }));
    }

    let img = CaptchaService::new().create(uuid);

    ImageResponse::Ok(img, ContentType::PNG)
}

#[post("/login", data = "<login>")]
fn login(login: Json<LoginDto>, mut log: SysLogLoginEntity) -> Json<ResponseWrapper> {
    if !CaptchaService::new().verify(login.uuid.clone(), login.captcha.clone()) {
        return Json(ResponseWrapper {
            code: ErrorCode::CaptchaError as i32,
            msg: "Captcha Error".to_string(),
            data: None,
        });
    }

    log.operation = 0; // 0:登录 1:登出

    let user = SysUserService::get_by_username(login.username.clone());
    if user.is_none() {
        log.status = 0; // 0:失败 1:成功 2:账号已锁定
        log.creator_name = login.username.clone();
        SysLogLoginService::save(log);
        return Json(ResponseWrapper {
            code: ErrorCode::AccountPasswordError as i32,
            msg: "Account or Password Error".to_string(),
            data: None,
        });
    }

    let user = user.unwrap();

    log.creator_name = login.username.clone();
    log.creator = user.id;

    if !bcrypt::verify(&login.password, &user.password) {
        log.status = 0; // 0:失败 1:成功 2:账号已锁定

        SysLogLoginService::save(log);
        return Json(ResponseWrapper {
            code: ErrorCode::AccountPasswordError as i32,
            msg: "Account or Password Error".to_string(),
            data: None,
        });
    }

    if user.status == 2 {
        log.status = 2; // 0:失败 1:成功 2:账号已锁定
        SysLogLoginService::save(log);
        return Json(ResponseWrapper {
            code: ErrorCode::AccountDisable as i32,
            msg: "Account Locked".to_string(),
            data: None,
        });
    }

    log.status = 1; // 0:失败 1:成功 2:账号已锁定
    SysLogLoginService::save(log);
    Json(SysUserTokenService::create_token(user.id))
}

#[post("/logout")]
fn logout(user: SysUserEntity, mut log: SysLogLoginEntity) -> Json<ResponseWrapper> {
    SysUserTokenService::logout(user.id);

    log.operation = 1; // 0:登录 1:登出
    log.status = 1; // 0:失败 1:成功 2:账号已锁定
    log.creator = user.id;
    log.creator_name = user.username;
    SysLogLoginService::save(log);

    Json(ResponseWrapper {
        code: 0,
        msg: "Success".to_string(),
        data: None,
    })
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Login", |rocket| async {
        rocket.mount("/renren-admin", routes![captcha, login, logout])
    })
}
