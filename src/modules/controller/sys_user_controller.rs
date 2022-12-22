use pwhash::bcrypt;
use rocket::{
    delete, fairing::AdHoc, get, http::ContentType, post, put, routes, serde::json::Json,
};

use crate::{
    common::{
        entity::sys_user_entity::SysUserEntity,
        utils::{page_data::PageData, ReqParams},
    },
    modules::{
        dto::{password_dto::PasswordDto, sys_user_dto::SysUserDto},
        services::{
            sys_dept_service::SysDeptService, sys_role_user_service::SysRoleUserService,
            sys_user_service::SysUserService,
        },
        ErrorCode, ResponseWrapper,
    },
};

#[get("/info")]
async fn info(user: SysUserEntity) -> Json<ResponseWrapper> {
    let user_dto = SysUserDto::from(Some(user)).unwrap();

    Json(ResponseWrapper::new(
        0,
        "success".to_string(),
        Some(serde_json::to_value(user_dto).unwrap()),
    ))
}

#[rocket_grants::has_permissions("sys:user:page")]
#[get("/page")]
async fn page(mut params: ReqParams, user: SysUserEntity) -> Json<ResponseWrapper> {
    if user.super_admin == 0 {
        let dept_id_list = SysDeptService::get_sub_dept_id_list(user.dept_id.unwrap());
        params.params.insert(
            "dept_id_list".to_string(),
            dept_id_list
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(","),
        );
    }

    let page: PageData<SysUserDto> = SysUserService::page(params).await;

    Json(ResponseWrapper::new(
        0,
        "success".to_string(),
        Some(serde_json::to_value(page).unwrap()),
    ))
}

#[rocket_grants::has_permissions("sys:user:delete")]
#[delete("/", data = "<ids>")]
async fn delete(ids: Json<Vec<String>>, _user: SysUserEntity) -> Json<ResponseWrapper> {
    let ids: Vec<i64> = ids
        .into_inner()
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    SysUserService::delete_batch_ids(ids).await;

    Json(ResponseWrapper::new(0, "success".to_string(), None))
}

#[rocket_grants::has_permissions("sys:user:export")]
#[get("/export")]
async fn export(mut params: ReqParams, user: SysUserEntity) -> (ContentType, Vec<u8>) {
    if user.super_admin == 0 {
        let dept_id_list = SysDeptService::get_sub_dept_id_list(user.dept_id.unwrap());
        params.params.insert(
            "dept_id_list".to_string(),
            dept_id_list
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(","),
        );
    }

    let list = SysUserService::list(params);

    let mut ret_str = "用户名,姓名,性别,邮箱,手机号,部门名称,状态,备注,创建时间\n".to_string();

    for item in list {
        ret_str.push_str(&format!(
            "{},{},{},{},{},{},{},{},{}\n",
            item.username,
            item.real_name,
            if item.gender == 1 { "男" } else { "女" },
            item.email,
            item.mobile,
            if item.dept_name.is_some() {
                item.dept_name.unwrap()
            } else {
                "".to_string()
            },
            if item.status == 1 { "正常" } else { "禁用" },
            "",
            item.create_date.format("%Y-%m-%d %H:%M:%S")
        ));
    }

    (ContentType::CSV, ret_str.into_bytes())
}

#[put("/password", data = "<dto>")]
async fn password(dto: Json<PasswordDto>, user: SysUserEntity) -> Json<ResponseWrapper> {
    if !bcrypt::verify(&dto.password, &user.password) {
        return Json(ResponseWrapper::new(
            ErrorCode::PasswordError as i32,
            "原密码错误".to_string(),
            None,
        ));
    }

    SysUserService::update_password(user.id, dto.new_password.clone()).await;

    Json(ResponseWrapper::new(0, "success".to_string(), None))
}

#[rocket_grants::has_permissions("sys:user:info")]
#[get("/<id>")]
async fn get(id: i64) -> Json<ResponseWrapper> {
    let mut user = SysUserService::get(id).await;

    let role_id_list = SysRoleUserService::get_role_id_list(id);

    user.role_id_list = role_id_list;

    Json(ResponseWrapper::new(
        0,
        "success".to_string(),
        Some(serde_json::to_value(user).unwrap()),
    ))
}

#[rocket_grants::has_permissions("sys:user:update")]
#[put("/", data = "<dto>")]
async fn update(dto: Json<SysUserDto>, user: SysUserEntity) -> Json<ResponseWrapper> {
    SysUserService::update(dto.into_inner(), user).await;

    Json(ResponseWrapper::new(0, "success".to_string(), None))
}

#[rocket_grants::has_permissions("sys:user:save")]
#[post("/", data = "<dto>")]
async fn save(dto: Json<SysUserDto>, user: SysUserEntity) -> Json<ResponseWrapper> {
    SysUserService::save(dto.into_inner(), user).await;

    Json(ResponseWrapper::new(0, "success".to_string(), None))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("User info", |rocket| async {
        rocket.mount(
            "/renren-admin/sys/user/",
            routes![info, page, delete, export, password, get, update, save],
        )
    })
}
