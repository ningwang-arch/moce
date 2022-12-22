use crate::{
    common::entity::sys_role_menu_entity::SysRoleMenuEntity,
    modules::dao::sys_role_menu_dao::SysRoleMenuDao,
};

pub struct SysRoleMenuService;
impl SysRoleMenuService {
    pub fn save_or_update(role_id: i64, menu_id_list: Vec<i64>, id: i64) {
        let role_ids = vec![role_id];
        SysRoleMenuDao::delete_by_role_ids(role_ids);
        if menu_id_list.is_empty() {
            return;
        }
        for menu_id in menu_id_list {
            let entity = SysRoleMenuEntity {
                id: 0,
                role_id,
                menu_id,
                creator: id,
                create_date: chrono::Local::now().naive_local(),
            };

            SysRoleMenuDao::insert(entity);
        }
    }

    pub fn get_menu_id_list(id: i64) -> Vec<i64> {
        SysRoleMenuDao::get_menu_id_list(id)
    }

    pub fn delete_by_role_ids(ids: Vec<i64>) {
        SysRoleMenuDao::delete_by_role_ids(ids);
    }

    pub fn delete_by_menu_id(id: i64) {
        SysRoleMenuDao::delete_by_menu_id(id);
    }
}
