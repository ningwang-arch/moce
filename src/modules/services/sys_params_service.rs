use std::collections::HashMap;

use crate::{common::utils::page_data::PageData, modules::dto::sys_params_dto::SysParamsDto};

pub struct SysParamsService;

impl SysParamsService {
    pub fn page(params: HashMap<String, String>) -> PageData<SysParamsDto> {
        todo!()
    }
}
