use std::collections::HashMap;

use rocket::{delete, fairing::AdHoc, get, post, put, routes, serde::json::Json};

use crate::{
    common::entity::sys_user_entity::SysUserEntity,
    modules::{
        dto::sys_dept_dto::SysDeptDto, services::sys_dept_service::SysDeptService, ResponseWrapper,
    },
};

#[rocket_grants::has_permissions("sys:dept:list")]
#[get("/list")]
async fn list(user: SysUserEntity) -> Json<ResponseWrapper> {
    let mut map = HashMap::new();
    if user.super_admin == 0 && user.dept_id.is_some() {
        let dept_list = SysDeptService::get_sub_dept_id_list(user.dept_id.unwrap());
        map.insert(
            "deptIdList".to_string(),
            dept_list
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<String>>()
                .join(","),
        );
    }

    let list: Vec<SysDeptDto> = SysDeptService::list(map);
    Json(ResponseWrapper::new(
        0,
        "success".to_string(),
        Some(serde_json::to_value(list).unwrap()),
    ))
}

#[rocket_grants::has_permissions("sys:dept:info")]
#[get("/<id>")]
async fn info(id: i64) -> Json<ResponseWrapper> {
    let dept = SysDeptService::get(id);
    Json(ResponseWrapper::new(
        0,
        "success".to_string(),
        Some(serde_json::to_value(dept).unwrap()),
    ))
}

#[rocket_grants::has_permissions("sys:dept:update")]
#[put("/", data = "<entity>")]
async fn update(entity: Json<SysDeptDto>, user: SysUserEntity) -> Json<ResponseWrapper> {
    SysDeptService::update(entity.into_inner(), user.id)
    //Json(ResponseWrapper::new(0, "success".to_string(), None))
}

#[rocket_grants::has_permissions("sys:dept:delete")]
#[delete("/<id>")]
async fn delete(id: i64) -> Json<ResponseWrapper> {
    SysDeptService::delete(id)
}

#[rocket_grants::has_permissions("sys:dept:save")]
#[post("/", data = "<dto>")]
async fn save(dto: Json<SysDeptDto>, user: SysUserEntity) -> Json<ResponseWrapper> {
    SysDeptService::save(dto.into_inner(), user.id);
    Json(ResponseWrapper::new(0, "success".to_string(), None))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Dept Controller", |rocket| async {
        rocket.mount(
            "/renren-admin/sys/dept",
            routes![list, info, update, delete, save],
        )
    })
}
