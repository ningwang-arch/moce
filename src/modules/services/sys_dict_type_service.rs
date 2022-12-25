use std::collections::HashMap;

use crate::{
    common::{
        entity::{
            dict_type::DictType, sys_dict_type_entity::SysDictTypeEntity,
            sys_user_entity::SysUserEntity,
        },
        utils::page_data::PageData,
    },
    modules::{
        dao::{sys_dict_data_dao::SysDictDataDao, sys_dict_type_dao::SysDictTypeDao},
        dto::sys_dict_type_dto::SysDictTypeDto,
    },
};

pub struct SysDictTypeService;

impl SysDictTypeService {
    pub fn get_all_list() -> Vec<DictType> {
        let mut type_list = SysDictTypeDao::get_dict_type_list();
        let data_list = SysDictDataDao::get_dict_data_list();

        for type_item in type_list.iter_mut() {
            for data_item in data_list.iter() {
                if type_item.id == data_item.dict_type_id {
                    type_item.data_list.push(data_item.clone());
                }
            }
        }
        type_list
    }

    pub fn page(params: HashMap<String, String>) -> PageData<SysDictTypeDto> {
        SysDictTypeDao::page(params)
    }

    pub fn save(dto: SysDictTypeDto, user: SysUserEntity) {
        let mut entity = SysDictTypeEntity::from(&dto);
        entity.creator = user.id;
        entity.updater = user.id;
        entity.create_date = chrono::Local::now().naive_local();
        entity.update_date = chrono::Local::now().naive_local();

        SysDictTypeDao::insert(entity);
    }

    pub fn get(id: i64) -> SysDictTypeDto {
        let entity = SysDictTypeDao::get(id);
        SysDictTypeDto::from(&entity)
    }

    pub fn update(dto: SysDictTypeDto, user: SysUserEntity) {
        let mut entity = SysDictTypeEntity::from(&dto);
        entity.updater = user.id;
        entity.update_date = chrono::Local::now().naive_local();

        SysDictTypeDao::update_by_id(entity);
    }

    pub fn delete(ids: Vec<i64>) {
        SysDictTypeDao::delete_batch_ids(ids);
    }
}
