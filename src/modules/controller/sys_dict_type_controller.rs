use rocket::{delete, fairing::AdHoc, get, post, put, routes, serde::json::Json};

use crate::{
    common::{entity::sys_user_entity::SysUserEntity, utils::ReqParams},
    modules::{
        dto::sys_dict_type_dto::SysDictTypeDto,
        services::sys_dict_type_service::SysDictTypeService, ResponseWrapper,
    },
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

#[rocket_grants::has_permissions("sys:dict:page")]
#[get("/page")]
async fn page(params: ReqParams) -> Json<ResponseWrapper> {
    let data = SysDictTypeService::page(params.params);
    Json(ResponseWrapper::new(
        0,
        "success".to_string(),
        Some(serde_json::to_value(data).unwrap()),
    ))
}

#[rocket_grants::has_permissions("sys:dict:save")]
#[post("/", data = "<dto>")]
async fn save(dto: Json<SysDictTypeDto>, user: SysUserEntity) -> Json<ResponseWrapper> {
    SysDictTypeService::save(dto.into_inner(), user);
    Json(ResponseWrapper::new(0, "success".to_string(), None))
}

#[rocket_grants::has_permissions("sys:dict:info")]
#[get("/<id>")]
async fn get(id: i64) -> Json<ResponseWrapper> {
    let data = SysDictTypeService::get(id);
    Json(ResponseWrapper::new(
        0,
        "success".to_string(),
        Some(serde_json::to_value(data).unwrap()),
    ))
}

#[rocket_grants::has_permissions("sys:dict:update")]
#[put("/", data = "<dto>")]
async fn update(dto: Json<SysDictTypeDto>, user: SysUserEntity) -> Json<ResponseWrapper> {
    SysDictTypeService::update(dto.into_inner(), user);
    Json(ResponseWrapper::new(0, "success".to_string(), None))
}

#[rocket_grants::has_permissions("sys:dict:delete")]
#[delete("/", data = "<ids>")]
async fn delete(ids: Json<Vec<String>>) -> Json<ResponseWrapper> {
    let ids = ids
        .into_inner()
        .iter()
        .map(|id| id.parse::<i64>().unwrap())
        .collect();
    SysDictTypeService::delete(ids);
    Json(ResponseWrapper::new(0, "success".to_string(), None))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Dict type", |rocket| async {
        rocket.mount(
            "/renren-admin/sys/dict/type/",
            routes![all, page, save, get, update, delete],
        )
    })
}
