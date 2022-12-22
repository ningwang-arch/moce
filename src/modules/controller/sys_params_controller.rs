use rocket::{get, serde::json::Json};

use crate::{
    common::utils::ReqParams,
    modules::{services::sys_params_service::SysParamsService, ResponseWrapper},
};

#[rocket_grants::has_permissions("sys:params:page")]
#[get("/page")]
async fn page(params: ReqParams) -> Json<ResponseWrapper> {
    let data = SysParamsService::page(params.params);
    Json(ResponseWrapper::new(
        0,
        "success".to_string(),
        Some(serde_json::to_value(data).unwrap()),
    ))
}
