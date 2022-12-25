use rocket::{delete, fairing::AdHoc, get, post, put, routes, serde::json::Json};

use crate::{
    common::{entity::sys_user_entity::SysUserEntity, utils::ReqParams},
    modules::{
        dto::sys_params_dto::SysParamsDto, services::sys_params_service::SysParamsService,
        ResponseWrapper,
    },
};

#[rocket_grants::has_permissions("sys:params:page")]
#[get("/page")]
async fn page(params: ReqParams) -> Json<ResponseWrapper> {
    let data = SysParamsService::page(params.params);
    Json(ResponseWrapper::new(
        0,
        "success".to_string(),
        Some(serde_json::to_value(data).unwrap()),
    ))
}

#[rocket_grants::has_permissions("sys:params:delete")]
#[delete("/", data = "<ids>")]
async fn delete(ids: Json<Vec<String>>) -> Json<ResponseWrapper> {
    let ids = ids
        .into_inner()
        .iter()
        .map(|id| id.parse::<i64>().unwrap())
        .collect();
    SysParamsService::delete(ids);
    Json(ResponseWrapper::new(0, "success".to_string(), None))
}

#[rocket_grants::has_permissions("sys:params:save")]
#[post("/", data = "<dto>")]
async fn save(dto: Json<SysParamsDto>, user: SysUserEntity) -> Json<ResponseWrapper> {
    let dto = dto.into_inner();
    SysParamsService::save(dto, user);
    Json(ResponseWrapper::new(0, "success".to_string(), None))
}

#[rocket_grants::has_permissions("sys:params:info")]
#[get("/<id>")]
async fn get(id: i64) -> Json<ResponseWrapper> {
    let data = SysParamsService::get(id);
    Json(ResponseWrapper::new(
        0,
        "success".to_string(),
        Some(serde_json::to_value(data).unwrap()),
    ))
}

#[rocket_grants::has_permissions("sys:params:update")]
#[put("/", data = "<dto>")]
async fn update(dto: Json<SysParamsDto>, user: SysUserEntity) -> Json<ResponseWrapper> {
    let dto = dto.into_inner();
    SysParamsService::update(dto, user);
    Json(ResponseWrapper::new(0, "success".to_string(), None))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Role Controller", |rocket| async {
        rocket.mount(
            "/renren-admin/sys/params/",
            routes![page, delete, save, get, update],
        )
    })
}
