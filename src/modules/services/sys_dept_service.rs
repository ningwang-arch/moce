use std::collections::HashMap;

use rocket::serde::json::Json;

use crate::{
    common::entity::sys_dept_entity::SysDeptEntity,
    modules::{
        dao::{sys_dept_dao::SysDeptDao, sys_user_dao::SysUserDao},
        dto::{build, sys_dept_dto::SysDeptDto},
        ErrorCode, ResponseWrapper,
    },
};

pub struct SysDeptService;

impl SysDeptService {
    pub fn get_sub_dept_id_list(id: i64) -> Vec<i64> {
        let mut dept_id_list = SysDeptDao::get_sub_dept_id_list(id);
        dept_id_list.push(id);
        dept_id_list
    }

    pub fn list(map: HashMap<String, String>) -> Vec<SysDeptDto> {
        let list = SysDeptDao::get_list(map);

        let mut list: Vec<SysDeptDto> = list
            .into_iter()
            .map(|item| SysDeptDto::from(&item))
            .collect();

        let mut root = SysDeptDto {
            id: 0,
            pid: 0,
            name: "".to_string(),
            sort: 0,
            create_date: chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
            parent_name: None,
            children: vec![],
        };

        build(&mut root, &mut list);
        root.children
    }

    // fn build(root: &mut SysDeptDto, list: &mut Vec<SysDeptDto>) {
    //     if list.is_empty() {
    //         return;
    //     }

    //     let mut i = 0;
    //     while i < list.len() {
    //         let item = list.get(i).unwrap();
    //         if item.pid == root.id {
    //             root.children.push(item.clone());
    //             list.remove(i);
    //         } else {
    //             i += 1;
    //         }
    //     }

    //     for item in root.children.iter_mut() {
    //         Self::build(item, list);
    //     }
    // }

    pub fn get(id: i64) -> SysDeptDto {
        let dept = SysDeptDao::get(id);
        SysDeptDto::from(&dept)
    }

    pub fn update(dto: SysDeptDto, id: i64) -> Json<ResponseWrapper> {
        let mut entity = SysDeptEntity::from(&dto);
        if entity.id == entity.pid {
            return Json(ResponseWrapper {
                code: ErrorCode::SuperiorDeptError as i32,
                msg: "上级部门不能是自己".to_string(),
                data: None,
            });
        }
        let sub_dept_list = SysDeptDao::get_sub_dept_id_list(entity.id);
        if sub_dept_list.contains(&entity.pid) {
            return Json(ResponseWrapper {
                code: ErrorCode::SuperiorDeptError as i32,
                msg: "上级部门不能是自己的子部门".to_string(),
                data: None,
            });
        }

        entity.pids = Self::get_pid_list(entity.pid);
        entity.update_date = chrono::Local::now().naive_local();
        entity.updater = id;
        SysDeptDao::update_by_id(entity);
        Json(ResponseWrapper {
            code: 0,
            msg: "更新成功".to_string(),
            data: None,
        })
    }

    fn get_pid_list(pid: i64) -> String {
        if pid == 0 {
            return "".to_string();
        }

        let dept_list: Vec<SysDeptEntity> = SysDeptDao::get_all();
        let map = dept_list
            .into_iter()
            .map(|item| (item.id, item))
            .collect::<HashMap<i64, SysDeptEntity>>();

        let mut pid_list = vec![];
        Self::get_pid_tree(pid, map, &mut pid_list);

        pid_list
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn get_pid_tree(pid: i64, map: HashMap<i64, SysDeptEntity>, pid_list: &mut Vec<i64>) {
        if pid == 0 {
            return;
        }

        if let Some(dept) = map.get(&pid) {
            pid_list.push(dept.id);
            Self::get_pid_tree(dept.pid, map, pid_list);
        }

        pid_list.push(pid);
    }

    pub fn delete(id: i64) -> Json<ResponseWrapper> {
        let dept_list = SysDeptDao::get_sub_dept_id_list(id);
        if !dept_list.is_empty() {
            return Json(ResponseWrapper {
                code: ErrorCode::DeptSubDeleteError as i32,
                msg: "该部门下有子部门，不能删除".to_string(),
                data: None,
            });
        }

        let count = SysUserDao::get_count_by_dept_id(id);
        if count > 0 {
            return Json(ResponseWrapper {
                code: ErrorCode::DeptUserDeleteError as i32,
                msg: "该部门下有用户，不能删除".to_string(),
                data: None,
            });
        }

        SysDeptDao::delete_by_id(id);

        Json(ResponseWrapper {
            code: 0,
            msg: "删除成功".to_string(),
            data: None,
        })
    }

    pub fn save(dto: SysDeptDto, id: i64) {
        let mut entity = SysDeptEntity::from(&dto);
        entity.create_date = chrono::Local::now().naive_local();
        entity.creator = id;
        entity.updater = id;
        entity.update_date = chrono::Local::now().naive_local();
        entity.pids = Self::get_pid_list(entity.pid);
        SysDeptDao::save(entity);
    }
}
