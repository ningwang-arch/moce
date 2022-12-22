use mysql::{params, prelude::Queryable};

use crate::{common::entity::sys_menu_entity::SysMenuEntity, modules::get_mysql_conn};

pub struct SysMenuDao;

impl SysMenuDao {
    pub fn get_menu_list(menu_type: Option<i32>) -> Vec<SysMenuEntity> {
        let mut conn = get_mysql_conn();
        if menu_type.is_some() {
            conn.exec::<SysMenuEntity, _, _>(
                "select t1.*, (select name from sys_menu t2 where t2.id=t1.pid) as parent_name from sys_menu t1  where t1.menu_type = :menu_type order by t1.sort asc",
                params! {"menu_type" => menu_type},
            )
            .unwrap()
        } else {
            conn.exec::<SysMenuEntity, _, _>(
                "select t1.*, (select name from sys_menu t2 where t2.id=t1.pid) as parent_name from sys_menu t1  order by t1.sort asc",
                (),
            )
            .unwrap()
        }
    }

    pub fn get_user_menu_list(user_id: i64, menu_type: Option<i32>) -> Vec<SysMenuEntity> {
        let mut conn = get_mysql_conn();

        if menu_type.is_some() {
            conn.exec::<SysMenuEntity, _, _>(
                "select t1.* , (select name from sys_menu t2 where t2.id=t1.pid) as parent_name from sys_menu t1
                inner join sys_role_menu t2 on t1.id = t2.menu_id
                inner join sys_user_role t3 on t2.role_id = t3.role_id
                where t3.user_id = :user_id and t1.menu_type = :menu_type
                order by t1.sort asc",
                params! {"user_id" => user_id, "menu_type" => menu_type},
            )
            .unwrap()
        } else {
            conn.exec::<SysMenuEntity, _, _>(
                "select t1.* ,(select name from sys_menu t2 where t2.id=t1.pid) as parent_name from sys_menu t1
                inner join sys_role_menu t2 on t1.id = t2.menu_id
                inner join sys_user_role t3 on t2.role_id = t3.role_id
                where t3.user_id = :user_id
                order by t1.sort asc",
                params! {"user_id" => user_id},
            )
            .unwrap()
        }
    }

    pub fn get_permissions_list() -> Vec<String> {
        let mut conn = get_mysql_conn();
        let sql = "select permissions from sys_menu where permissions is not null";

        conn.exec(sql, ()).unwrap()
    }

    pub fn get_user_permissions_list(user_id: i64) -> Vec<String> {
        let mut conn = get_mysql_conn();
        let sql = "select t3.permissions from sys_role_user t1 left join sys_role_menu t2 on t1.role_id = t2.role_id
			left join sys_menu t3 on t2.menu_id = t3.id
		where t1.user_id = :user_d order by t3.sort asc";

        conn.exec(sql, params! {"user_id" => user_id}).unwrap()
    }

    pub fn get_by_id(id: i64) -> Option<SysMenuEntity> {
        let mut conn = get_mysql_conn();
        let sql = "select t1.*, (select name from sys_menu t2 where t2.id=t1.pid) as parent_name from sys_menu t1
			where t1.id = :id";

        conn.exec_first(sql, params! {"id" => id}).unwrap()
    }

    pub fn insert(entity: SysMenuEntity) {
        let mut conn = get_mysql_conn();

        let sql = "insert into sys_menu (pid, name, url, permissions, menu_type, icon, sort, creator, create_date, updater, update_date) values (:pid, :name, :url, :permissions, :menu_type, :icon, :sort, :creator, :create_date, :updater, :update_date)";

        conn.exec_drop(
            sql,
            params! {
                "pid" => entity.pid,
                "name" => entity.name,
                "url" => entity.url,
                "permissions" => entity.perms,
                "menu_type" => entity.menu_type,
                "icon" => entity.icon,
                "sort" => entity.sort,
                "creator" => entity.creator,
                "create_date" => entity.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                "updater" => entity.updater,
                "update_date" => entity.update_time.format("%Y-%m-%d %H:%M:%S").to_string()
            },
        )
        .unwrap();
    }

    pub fn get_list_pid(id: i64) -> Vec<SysMenuEntity> {
        let mut conn = get_mysql_conn();
        let sql = "select t1.*, (select name from sys_menu t2 where t2.id=t1.pid) as parent_name from sys_menu t1
            where t1.pid = :id";

        conn.exec(sql, params! {"id" => id}).unwrap()
    }

    pub fn delete_by_id(id: i64) {
        let mut conn = get_mysql_conn();
        let sql = "delete from sys_menu where id = :id";

        conn.exec_drop(sql, params! {"id" => id}).unwrap();
    }

    pub fn update(entity: SysMenuEntity) {
        let mut conn = get_mysql_conn();

        let sql = "update sys_menu set pid = :pid, name = :name, url = :url, permissions = :permissions, menu_type = :menu_type, icon = :icon, sort = :sort, updater = :updater, update_date = :update_date where id = :id";

        conn.exec_drop(
            sql,
            params! {
                "pid" => entity.pid,
                "name" => entity.name,
                "url" => entity.url,
                "permissions" => entity.perms,
                "menu_type" => entity.menu_type,
                "icon" => entity.icon,
                "sort" => entity.sort,
                "updater" => entity.updater,
                "update_date" => entity.update_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                "id" => entity.id
            },
        )
        .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::modules::dao::sys_menu_dao::SysMenuDao;

    #[test]
    fn test_get_menu_list() {
        let result = SysMenuDao::get_menu_list(Some(0));
        println!("{:?}", result);
        println!("size: {}", result.len());
    }

    #[test]
    fn test_get_user_menu_list() {
        let result = SysMenuDao::get_user_menu_list(1067246875800000001, Some(0));
        println!("{:?}", result);
    }
}
