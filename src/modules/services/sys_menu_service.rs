use std::collections::HashSet;

use rocket::serde::json::Json;

use crate::{
    common::entity::{sys_menu_entity::SysMenuEntity, sys_user_entity::SysUserEntity},
    modules::{
        dao::sys_menu_dao::SysMenuDao,
        dto::{build, sys_menu_dto::SysMenuDto},
        ErrorCode, ResponseWrapper,
    },
};

use super::sys_role_menu_service::SysRoleMenuService;

pub struct SysMenuService;

impl SysMenuService {
    pub fn get_user_menu_list(user: SysUserEntity, menu_type: Option<i32>) -> Vec<SysMenuDto> {
        let menu_list = if user.super_admin == 1 {
            SysMenuDao::get_menu_list(menu_type)
        } else {
            SysMenuDao::get_user_menu_list(user.id, menu_type)
        };

        let mut dto_list: Vec<SysMenuDto> = menu_list.into_iter().map(SysMenuDto::from).collect();

        let mut root = SysMenuDto {
            id: 0,
            pid: 0,
            name: "".to_string(),
            url: None,
            menu_type: 0,
            icon: None,
            sort: 0,
            permissions: None,
            create_date: chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
            parent_name: None,
            children: vec![],
        };

        build(&mut root, &mut dto_list);
        root.children
    }

    pub fn get_user_permissions(user: SysUserEntity) -> HashSet<String> {
        let permissions_list = if user.super_admin == 1 {
            SysMenuDao::get_permissions_list()
        } else {
            SysMenuDao::get_user_permissions_list(user.id)
        };

        let mut perm_list = HashSet::new();
        for item in &permissions_list {
            item.trim().split(',').for_each(|x| {
                perm_list.insert(x.to_string());
            });
        }
        perm_list
    }

    pub fn get_all_menu_list(r#type: Option<i32>) -> Vec<SysMenuDto> {
        let menu_list = SysMenuDao::get_menu_list(r#type);

        let mut dto_list: Vec<SysMenuDto> = menu_list.into_iter().map(SysMenuDto::from).collect();

        let mut root = SysMenuDto {
            id: 0,
            pid: 0,
            name: "".to_string(),
            url: None,
            menu_type: 0,
            icon: None,
            sort: 0,
            permissions: None,
            create_date: chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
            parent_name: None,
            children: vec![],
        };

        build(&mut root, &mut dto_list);
        root.children
    }

    pub fn get(id: i64) -> SysMenuDto {
        let menu = SysMenuDao::get_by_id(id);
        if menu.is_none() {
            panic!("菜单不存在");
        }
        SysMenuDto::from(menu.unwrap())
    }

    pub fn save(dto: SysMenuDto, user: SysUserEntity) {
        let mut entity = SysMenuEntity::from(dto);
        entity.creator = user.id;
        entity.updater = user.id;

        SysMenuDao::insert(entity);
    }

    pub fn get_list_pid(id: i64) -> Vec<SysMenuDto> {
        let menu_list = SysMenuDao::get_list_pid(id);
        menu_list.into_iter().map(SysMenuDto::from).collect()
    }

    pub fn delete(id: i64) {
        SysMenuDao::delete_by_id(id);

        SysRoleMenuService::delete_by_menu_id(id);
    }

    pub fn update(dto: SysMenuDto, user: SysUserEntity) -> Json<ResponseWrapper> {
        let mut entity = SysMenuEntity::from(dto);

        if entity.id == entity.pid {
            return Json(ResponseWrapper {
                code: ErrorCode::SuperiorMenuError as i32,
                msg: "上级菜单不能是自己".to_string(),
                data: None,
            });
        }

        entity.updater = user.id;

        SysMenuDao::update(entity);

        Json(ResponseWrapper {
            code: 0,
            msg: "success".to_string(),
            data: None,
        })
    }
}
