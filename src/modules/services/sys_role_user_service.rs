use chrono::Local;

use crate::{
    common::entity::sys_role_user_entity::SysRoleUserEntity,
    modules::dao::sys_role_user_dao::SysRoleUserDao,
};

pub struct SysRoleUserService;

impl SysRoleUserService {
    pub fn get_role_id_list(id: i64) -> Vec<i64> {
        SysRoleUserDao::get_role_id_list(id)
    }

    pub fn save_or_update(id: i64, role_id_list: Vec<i64>, creator: i64) {
        SysRoleUserDao::delete_by_user_ids(vec![id]);

        if role_id_list.is_empty() {
            return;
        }

        for role_id in role_id_list {
            let entity = SysRoleUserEntity {
                id: 0,
                role_id,
                user_id: id,
                creator,
                create_time: Local::now().naive_local(),
            };
            SysRoleUserDao::insert(entity);
        }
    }

    pub fn delete_by_role_ids(ids: Vec<i64>) {
        SysRoleUserDao::delete_by_role_ids(ids);
    }
}
