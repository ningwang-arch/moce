use rocket::{delete, fairing::AdHoc, get, post, put, routes, serde::json::Json};

use crate::{
    common::{entity::sys_user_entity::SysUserEntity, utils::perms::save_permissions},
    modules::{
        dto::sys_menu_dto::SysMenuDto, services::sys_menu_service::SysMenuService, ErrorCode,
        ResponseWrapper,
    },
};

#[get("/nav")]
fn nav(user: SysUserEntity) -> Json<ResponseWrapper> {
    let list = SysMenuService::get_user_menu_list(user, Some(0));

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

#[get("/permissions")]
fn permissions(user: SysUserEntity) -> Json<ResponseWrapper> {
    let list = SysMenuService::get_user_permissions(user.clone());

    save_permissions(user.id, list.clone());

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

#[rocket_grants::has_permissions("sys:menu:select")]
#[get("/select")]
async fn select(user: SysUserEntity) -> Json<ResponseWrapper> {
    let list = SysMenuService::get_user_menu_list(user, None);
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

#[rocket_grants::has_permissions("sys:menu:list")]
#[get("/list?<type>")]
async fn list(r#type: Option<i32>) -> Json<ResponseWrapper> {
    let list = SysMenuService::get_all_menu_list(r#type);
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

#[rocket_grants::has_permissions("sys:menu:info")]
#[get("/<id>")]
async fn get(id: i64) -> Json<ResponseWrapper> {
    let data = SysMenuService::get(id);
    Json(ResponseWrapper::new(
        0,
        "success".to_string(),
        Some(serde_json::to_value(data).unwrap()),
    ))
}

#[rocket_grants::has_permissions("sys:menu:save")]
#[post("/", data = "<menu>")]
async fn save(menu: Json<SysMenuDto>, user: SysUserEntity) -> Json<ResponseWrapper> {
    SysMenuService::save(menu.into_inner(), user);
    Json(ResponseWrapper::new(0, "success".to_string(), None))
}

#[rocket_grants::has_permissions("sys:menu:delete")]
#[delete("/<id>")]
async fn delete(id: i64) -> Json<ResponseWrapper> {
    let list = SysMenuService::get_list_pid(id);
    if !list.is_empty() {
        return Json(ResponseWrapper::new(
            ErrorCode::SubMenuExist as i32,
            "请先删除子菜单或按钮".to_string(),
            None,
        ));
    }

    SysMenuService::delete(id);
    Json(ResponseWrapper::new(0, "success".to_string(), None))
}

#[rocket_grants::has_permissions("sys:menu:update")]
#[put("/", data = "<menu>")]
async fn update(menu: Json<SysMenuDto>, user: SysUserEntity) -> Json<ResponseWrapper> {
    SysMenuService::update(menu.into_inner(), user)
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Menu", |rocket| async {
        rocket.mount(
            "/renren-admin/sys/menu/",
            routes![nav, permissions, select, list, get, save, delete, update],
        )
    })
}
