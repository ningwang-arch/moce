use std::collections::HashSet;

use crate::common::redis::{redis_keys::get_user_permissions_key, redis_utils::RedisUtils};

pub fn save_permissions(user_id: i64, permissions: HashSet<String>) {
    let key = get_user_permissions_key(user_id);
    let perms_str = permissions
        .iter()
        .cloned()
        .collect::<Vec<String>>()
        .join(",");
    // save to redis
    let mut redis_conn = RedisUtils::new();

    redis_conn.set(&key, &perms_str, 3600).unwrap();
}

pub enum Logical {
    AND,
    OR,
}

pub fn check_permissions(user_id: i64, permissions: HashSet<String>, relation: Logical) -> bool {
    let key = get_user_permissions_key(user_id);
    let mut redis_conn = RedisUtils::new();
    let perms_str = redis_conn.get(&key, 3600).unwrap();
    let perms: Vec<String> = perms_str.split(',').map(|s| s.to_string()).collect();

    match relation {
        Logical::AND => {
            for perm in permissions {
                if !perms.contains(&perm) {
                    return false;
                }
            }
            true
        }
        Logical::OR => {
            for perm in permissions {
                if perms.contains(&perm) {
                    return true;
                }
            }
            false
        }
    }
}
