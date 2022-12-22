use crate::{
    common::entity::dict_type::DictType,
    modules::dao::{sys_dict_data_dao::SysDictDataDao, sys_dict_type_dao::SysDictTypeDao},
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
}
