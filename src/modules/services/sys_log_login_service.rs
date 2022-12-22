use crate::{
    common::entity::sys_log_login_entity::SysLogLoginEntity,
    modules::dao::sys_log_login_dao::SysLogLoginDao,
};

pub struct SysLogLoginService;

impl SysLogLoginService {
    pub fn save(log: SysLogLoginEntity) {
        SysLogLoginDao::insert(log);
    }
}
