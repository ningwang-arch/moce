use std::collections::HashMap;

use rocket::{delete, fairing::AdHoc, get, post, put, routes, serde::json::Json};

use crate::{
    common::{entity::sys_user_entity::SysUserEntity, utils::ReqParams},
    modules::{
        dto::sys_role_dto::SysRoleDto,
        services::{
            sys_dept_service::SysDeptService, sys_role_data_scope_service::SysRoleDataScopeService,
            sys_role_menu_service::SysRoleMenuService, sys_role_service::SysRoleService,
        },
        ErrorCode, ResponseWrapper,
    },
};

#[rocket_grants::has_permissions("sys:role:page")]
#[get("/page")]
async fn page(mut params: ReqParams, user: SysUserEntity) -> Json<ResponseWrapper> {
    if user.super_admin == 0 && user.dept_id.is_some() {
        let dept_id_list = SysDeptService::get_sub_dept_id_list(user.dept_id.unwrap());
        let dept_id_list = dept_id_list
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");
        params.params.insert("deptIdList".to_string(), dept_id_list);
    }

    let data = SysRoleService::page(params.params);

    Json(ResponseWrapper {
        code: 0,
        msg: "success".to_string(),
        data: Some(serde_json::to_value(data).unwrap()),
    })
}

#[rocket_grants::has_permissions("sys:role:save")]
#[post("/", data = "<dto>")]
async fn save(dto: Json<SysRoleDto>, user: SysUserEntity) -> Json<ResponseWrapper> {
    SysRoleService::save(dto.into_inner(), user);
    Json(ResponseWrapper {
        code: 0,
        msg: "success".to_string(),
        data: None,
    })
}

#[rocket_grants::has_permissions("sys:role:info")]
#[get("/<id>")]
async fn get(id: i64) -> Json<ResponseWrapper> {
    let mut data = SysRoleService::get(id);
    let menu_id_list = SysRoleMenuService::get_menu_id_list(id);
    data.menu_id_list = menu_id_list;

    let dept_id_list = SysRoleDataScopeService::get_dept_id_list(id);
    data.dept_id_list = dept_id_list;

    Json(ResponseWrapper {
        code: 0,
        msg: "success".to_string(),
        data: Some(serde_json::to_value(data).unwrap()),
    })
}

#[rocket_grants::has_permissions("sys:role:list")]
#[get("/list")]
async fn list(user: SysUserEntity) -> Json<ResponseWrapper> {
    let mut map: HashMap<String, String> = HashMap::new();
    if user.super_admin == 0 && user.dept_id.is_some() {
        let list = SysDeptService::get_sub_dept_id_list(user.dept_id.unwrap());
        let dept_id_list = list
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");
        map.insert("deptIdList".to_string(), dept_id_list);
    }
    let data = SysRoleService::list(map);
    Json(ResponseWrapper {
        code: 0,
        msg: "success".to_string(),
        data: Some(serde_json::to_value(data).unwrap()),
    })
}

#[rocket_grants::has_permissions("sys:role:update")]
#[put("/", data = "<dto>")]
async fn update(dto: Json<SysRoleDto>, user: SysUserEntity) -> Json<ResponseWrapper> {
    SysRoleService::update(dto.into_inner(), user);
    Json(ResponseWrapper {
        code: 0,
        msg: "success".to_string(),
        data: None,
    })
}

#[rocket_grants::has_permissions("sys:role:delete")]
#[delete("/", data = "<ids>")]
async fn delete(ids: Json<Vec<String>>) -> Json<ResponseWrapper> {
    if ids.is_empty() {
        return Json(ResponseWrapper {
            code: ErrorCode::NotNull as i32,
            msg: "参数错误".to_string(),
            data: None,
        });
    }

    let ids = ids
        .into_inner()
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    SysRoleService::delete(ids);
    Json(ResponseWrapper {
        code: 0,
        msg: "success".to_string(),
        data: None,
    })
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Role Controller", |rocket| async {
        rocket.mount(
            "/renren-admin/sys/role/",
            routes![page, save, get, list, update, delete],
        )
    })
}
