use crate::{
    common::entity::sys_role_data_scope_entity::SysRoleDataScopeEntity,
    modules::dao::sys_role_data_scope_dao::SysRoleDataScopeDao,
};

pub struct SysRoleDataScopeService;

impl SysRoleDataScopeService {
    pub fn save_or_update(role_id: i64, dept_id_list: Vec<i64>, id: i64) {
        let role_ids = vec![role_id];
        SysRoleDataScopeDao::delete_by_role_ids(role_ids);
        if dept_id_list.is_empty() {
            return;
        }
        for dept_id in dept_id_list {
            let entity = SysRoleDataScopeEntity {
                id: 0,
                role_id,
                dept_id,
                creator: id,
                create_date: chrono::Local::now().naive_local(),
            };

            SysRoleDataScopeDao::insert(entity);
        }
    }

    pub fn get_dept_id_list(id: i64) -> Vec<i64> {
        SysRoleDataScopeDao::get_dept_id_list(id)
    }

    pub fn delete_by_role_ids(ids: Vec<i64>) {
        SysRoleDataScopeDao::delete_by_role_ids(ids);
    }
}
