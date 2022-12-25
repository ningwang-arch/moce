use std::collections::HashMap;

use crate::{
    common::{
        entity::{sys_params_entity::SysParamsEntity, sys_user_entity::SysUserEntity},
        utils::{page_data::PageData, sys_params_redis::SysParamsRedis},
    },
    modules::{dao::sys_params_dao::SysParamsDao, dto::sys_params_dto::SysParamsDto},
};

pub struct SysParamsService;

impl SysParamsService {
    pub fn page(params: HashMap<String, String>) -> PageData<SysParamsDto> {
        SysParamsDao::page(params)
    }

    pub fn delete(ids: Vec<i64>) {
        let param_code_list = SysParamsDao::get_param_code_list(ids.clone());

        let mut redis = SysParamsRedis::new();
        redis.delete(param_code_list);

        SysParamsDao::delete_batch_ids(ids);
    }

    pub fn save(dto: SysParamsDto, user: SysUserEntity) {
        let mut entity = SysParamsEntity::from(&dto);
        entity.creator = user.id;
        entity.updater = user.id;

        SysParamsDao::insert(entity.clone());

        let mut redis = SysParamsRedis::new();
        redis.set(&entity.param_code, &entity.param_value);
    }

    pub fn get(id: i64) -> SysParamsDto {
        let entity = SysParamsDao::select_by_id(id);
        SysParamsDto::from(&entity)
    }

    pub fn update(dto: SysParamsDto, user: SysUserEntity) {
        let mut entity = SysParamsEntity::from(&dto);
        entity.updater = user.id;
        entity.update_date = chrono::Local::now().naive_local();

        SysParamsDao::update_by_id(entity.clone());

        let mut redis = SysParamsRedis::new();
        redis.set(&entity.param_code, &entity.param_value);
    }
}
