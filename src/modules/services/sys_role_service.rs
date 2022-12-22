use std::collections::HashMap;

use crate::{
    common::{
        entity::{sys_role_entity::SysRoleEntity, sys_user_entity::SysUserEntity},
        utils::page_data::PageData,
    },
    modules::{dao::sys_role_dao::SysRoleDao, dto::sys_role_dto::SysRoleDto},
};

use super::{
    sys_role_data_scope_service::SysRoleDataScopeService,
    sys_role_menu_service::SysRoleMenuService, sys_role_user_service::SysRoleUserService,
};

pub struct SysRoleService;
impl SysRoleService {
    pub fn page(params: HashMap<String, String>) -> PageData<SysRoleDto> {
        SysRoleDao::page(&params)
    }

    pub fn save(dto: SysRoleDto, user: SysUserEntity) {
        let mut entity = SysRoleEntity::from(dto.clone());
        entity.creator = user.id;
        entity.updater = user.id;

        let id = SysRoleDao::insert(entity);

        SysRoleMenuService::save_or_update(id, dto.menu_id_list, user.id);

        SysRoleDataScopeService::save_or_update(id, dto.dept_id_list, user.id);
    }

    pub fn get(id: i64) -> SysRoleDto {
        let entity = SysRoleDao::select_by_id(id);
        SysRoleDto::from(&entity)
    }

    pub fn list(map: HashMap<String, String>) -> Vec<SysRoleDto> {
        let list = SysRoleDao::select_list(&map);
        list.into_iter().map(|x| SysRoleDto::from(&x)).collect()
    }

    pub fn update(dto: SysRoleDto, user: SysUserEntity) {
        let mut entity = SysRoleEntity::from(dto.clone());
        entity.updater = user.id;
        entity.update_date = chrono::Local::now().naive_local();

        SysRoleDao::update_by_id(entity.clone());

        SysRoleMenuService::save_or_update(entity.id, dto.menu_id_list, user.id);

        SysRoleDataScopeService::save_or_update(entity.id, dto.dept_id_list, user.id);
    }

    pub fn delete(ids: Vec<i64>) {
        SysRoleDao::delete_batch_ids(ids.clone());

        SysRoleUserService::delete_by_role_ids(ids.clone());

        SysRoleMenuService::delete_by_role_ids(ids.clone());

        SysRoleDataScopeService::delete_by_role_ids(ids);
    }
}
