use rocket::{fairing::AdHoc, get, routes, serde::json::Json};

use crate::{
    common::entity::sys_user_entity::SysUserEntity,
    modules::{services::sys_dict_type_service::SysDictTypeService, ResponseWrapper},
};

#[get("/all")]
fn all(_user: SysUserEntity) -> Json<ResponseWrapper> {
    let list = SysDictTypeService::get_all_list();

    Json(ResponseWrapper::new(
        0,
        "success".to_string(),
        Some(serde_json::Value::Array(
            list.iter()
                .map(|item| serde_json::to_value(item).unwrap())
                .collect(),
        )),
    ))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Dict type", |rocket| async {
        rocket.mount("/renren-admin/sys/dict/type/", routes![all])
    })
}
