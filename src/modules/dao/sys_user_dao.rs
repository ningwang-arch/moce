use std::collections::HashMap;

use mysql::{params, prelude::Queryable};

use crate::{
    common::{
        entity::sys_user_entity::SysUserEntity,
        utils::page_data::{query_order, PageData},
    },
    modules::get_mysql_conn,
};

pub struct SysUserDao;

impl SysUserDao {
    pub fn get_by_username(username: String) -> Option<SysUserEntity> {
        let mut conn = get_mysql_conn();
        let sql = "select t1.*, (select t2.name from sys_dept t2 where t2.id=t1.dept_id) dept_name from sys_user t1 where username = :username";

        conn.exec_first::<SysUserEntity, _, _>(sql, params! {"username" => username})
            .unwrap()
    }

    pub fn get_by_user_id(id: i64) -> Option<SysUserEntity> {
        let mut conn = get_mysql_conn();
        let sql = "select t1.*, (select t2.name from sys_dept t2 where t2.id=t1.dept_id) dept_name from sys_user t1
			where t1.id = :id";

        conn.exec_first::<SysUserEntity, _, _>(sql, params! {"id" => id})
            .unwrap()
    }

    pub fn page(params: &HashMap<String, String>) -> PageData<SysUserEntity> {
        let mut conn = get_mysql_conn();
        let order_by = query_order(params, "create_date".to_string(), false);

        let mut where_sql = "where t1.super_admin = 0".to_string();

        if let Some(username) = params.get("username") {
            where_sql.push_str(&format!(" and t1.username like '%{}%'", username));
        }

        if let Some(dept_id) = params.get("deptId") {
            if !dept_id.trim().is_empty() {
                where_sql.push_str(&format!(" and t1.dept_id = {}", dept_id));
            }
        }

        if let Some(gender) = params.get("gender") {
            if !gender.trim().is_empty() {
                where_sql.push_str(&format!(" and t1.gender = {}", gender));
            }
        }

        if let Some(dept_id_list) = params.get("deptIdList") {
            if !dept_id_list.trim().is_empty() {
                where_sql.push_str(&format!(" and t1.dept_id in ({})", dept_id_list));
            }
        }

        // sql.push_str(order_by.as_str());

        let count_sql = format!(
            "select SQL_NO_CACHE count(*) from sys_user t1 {}",
            where_sql.as_str()
        );

        let select_sql = format!(
            "select SQL_NO_CACHE t1.*, (select t2.name from sys_dept t2 where t2.id=t1.dept_id) dept_name from sys_user t1 {} {}",
            where_sql.as_str(),
            order_by.as_str()
        );

        let count = conn
            .exec_first::<i64, _, _>(count_sql.as_str(), ())
            .unwrap()
            .unwrap();

        let mut list = conn
            .exec::<SysUserEntity, _, _>(select_sql.as_str(), ())
            .unwrap();
        for user in list.iter_mut() {
            user.password = "".to_string();
        }

        PageData::new(count, list)
    }

    pub async fn delete_batch_ids(ids: Vec<i64>) -> bool {
        let mut conn = get_mysql_conn();
        let ids: String = ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let sql = "delete from sys_user where id in (:ids)";

        conn.exec_drop(sql, params! {"ids" => ids}).is_ok()
    }

    pub fn get_list(params: &HashMap<String, String>) -> Vec<SysUserEntity> {
        let mut conn = get_mysql_conn();
        let mut sql = "select t1.*, (select t2.name from sys_dept t2 where t2.id=t1.dept_id) dept_name from sys_user t1 where t1.super_admin = 0".to_string();

        if let Some(username) = params.get("username") {
            sql.push_str(&format!(" and t1.username like '%{}%'", username));
        }

        if let Some(dept_id) = params.get("deptId") {
            if !dept_id.trim().is_empty() {
                sql.push_str(&format!(" and t1.dept_id = {}", dept_id));
            }
        }

        if let Some(gender) = params.get("gender") {
            if !gender.trim().is_empty() {
                sql.push_str(&format!(" and t1.gender = {}", gender));
            }
        }

        if let Some(dept_id_list) = params.get("deptIdList") {
            if !dept_id_list.trim().is_empty() {
                sql.push_str(&format!(" and t1.dept_id in ({})", dept_id_list));
            }
        }

        conn.exec::<SysUserEntity, _, _>(sql.as_str(), ()).unwrap()
    }

    pub fn update_password(id: i64, new_password: String) {
        let mut conn = get_mysql_conn();
        let sql = "update sys_user set password = :new_password where id = :id";

        conn.exec_drop(sql, params! {"new_password" => new_password, "id" => id})
            .unwrap();
    }

    pub fn get_count_by_dept_id(id: i64) -> i64 {
        let mut conn = get_mysql_conn();
        let sql = "select count(*) from sys_user where dept_id = :id";

        conn.exec_first::<i64, _, _>(sql, params! {"id" => id})
            .unwrap()
            .unwrap()
    }

    /*
        id          bigint auto_increment comment 'id'
        primary key,
    username    varchar(50)      not null comment '用户名',
    password    varchar(100)     null comment '密码',
    real_name   varchar(50)      null comment '姓名',
    head_url    varchar(200)     null comment '头像',
    gender      tinyint unsigned null comment '性别   0：男   1：女    2：保密',
    email       varchar(100)     null comment '邮箱',
    mobile      varchar(100)     null comment '手机号',
    dept_id     bigint           null comment '部门ID',
    super_admin tinyint unsigned null comment '超级管理员   0：否   1：是',
    status      tinyint          null comment '状态  0：停用   1：正常',
    creator     bigint           null comment '创建者',
    create_date datetime         null comment '创建时间',
    updater     bigint           null comment '更新者',
    update_date datetime         null comment '更新时间',
    */
    pub fn update_by_id(entity: SysUserEntity) {
        let mut conn = get_mysql_conn();
        let sql = "update sys_user set username = :username, password = :password, real_name = :realname, gender = :gender, email = :email, mobile = :mobile, dept_id = :dept_id, super_admin = :super_admin, status = :status, updater = :updater, update_date = :update_date where id = :id";

        conn.exec_drop(
            sql,
            params! {
                "username" => entity.username,
                "password" => entity.password,
                "realname" => entity.real_name,
                "gender"=>entity.gender,
                "email"=>entity.email,
                "mobile"=>entity.mobile,
                "dept_id"=>entity.dept_id,
                "super_admin"=>entity.super_admin,
                "status"=>entity.status,
                "updater"=>entity.updater,
                "update_date"=>entity.update_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                "id"=>entity.id
            },
        )
        .unwrap();
    }

    pub fn insert(entity: SysUserEntity) {
        let mut conn = get_mysql_conn();
        let sql = "insert into sys_user (username, password, real_name, gender, email, mobile, dept_id, super_admin, status, creator, create_date, updater, update_date) values (:username, :password, :realname, :gender, :email, :mobile, :dept_id, :super_admin, :status, :creator, :create_date, :updater, :update_date)";

        conn.exec_drop(
            sql,
            params! {
                "username" => entity.username,
                "password" => entity.password,
                "realname" => entity.real_name,
                "gender"=>  entity.gender,
                "email"=>   entity.email,
                "mobile"=>  entity.mobile,
                "dept_id"=> entity.dept_id,
                "super_admin"=> entity.super_admin,
                "status"=>  entity.status,
                "creator"=> entity.creator,
                "create_date"=> entity.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                "updater"=> entity.updater,
                "update_date"=> entity.update_time.format("%Y-%m-%d %H:%M:%S").to_string()
            },
        )
        .unwrap();
    }
}
