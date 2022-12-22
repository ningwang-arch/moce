use pwhash::bcrypt::{self, BcryptSetup, BcryptVariant};

use crate::{
    common::{
        entity::sys_user_entity::SysUserEntity,
        utils::{page_data::PageData, ReqParams},
    },
    modules::{dao::sys_user_dao::SysUserDao, dto::sys_user_dto::SysUserDto},
};

use super::sys_role_user_service::SysRoleUserService;

pub struct SysUserService;

impl SysUserService {
    pub fn get_by_username(username: String) -> Option<SysUserDto> {
        let user = SysUserDao::get_by_username(username);
        SysUserDto::from(user)
    }

    pub async fn page(params: ReqParams) -> PageData<SysUserDto> {
        let page: PageData<SysUserEntity> = SysUserDao::page(&params.params);
        let mut list = vec![];
        for user in page.list {
            let mut user_dto = SysUserDto::from(Some(user)).unwrap();
            user_dto.password = "".to_string();
            list.push(user_dto);
        }
        PageData::new(page.total, list)
    }

    pub async fn delete_batch_ids(ids: Vec<i64>) -> bool {
        SysUserDao::delete_batch_ids(ids).await
    }

    pub fn list(params: ReqParams) -> Vec<SysUserDto> {
        let list = SysUserDao::get_list(&params.params);

        let mut user_dto_list = vec![];
        for user in list {
            let mut user_dto = SysUserDto::from(Some(user)).unwrap();
            user_dto.password = "".to_string();
            user_dto_list.push(user_dto);
        }

        user_dto_list
    }

    pub async fn update_password(id: i64, password: String) {
        let new_password = bcrypt::hash_with(
            BcryptSetup {
                variant: Some(BcryptVariant::V2a),
                ..Default::default()
            },
            password.as_bytes(),
        )
        .unwrap();

        SysUserDao::update_password(id, new_password);
    }

    pub async fn get(id: i64) -> SysUserDto {
        let user = SysUserDao::get_by_user_id(id);
        let mut dto = SysUserDto::from(user).unwrap();
        dto.password = "".to_string();
        dto
    }

    pub async fn update(dto: SysUserDto, user: SysUserEntity) {
        let mut entity = SysUserEntity::from(&dto);
        entity.updater = user.id;
        entity.update_time = chrono::Local::now().naive_local();
        entity.password = bcrypt::hash_with(
            BcryptSetup {
                variant: Some(BcryptVariant::V2a),
                ..Default::default()
            },
            entity.password.as_bytes(),
        )
        .unwrap();

        SysUserDao::update_by_id(entity.clone());

        SysRoleUserService::save_or_update(entity.id, dto.role_id_list, user.id);
    }

    pub async fn save(dto: SysUserDto, user: SysUserEntity) {
        let mut entity = SysUserEntity::from(&dto);
        entity.creator = user.id;
        entity.create_time = chrono::Local::now().naive_local();
        entity.updater = user.id;
        entity.update_time = chrono::Local::now().naive_local();
        entity.password = bcrypt::hash_with(
            BcryptSetup {
                variant: Some(BcryptVariant::V2a),
                ..Default::default()
            },
            entity.password.as_bytes(),
        )
        .unwrap();
        entity.super_admin = 0;

        SysUserDao::insert(entity.clone());

        SysRoleUserService::save_or_update(entity.id, dto.role_id_list, user.id);
    }
}
