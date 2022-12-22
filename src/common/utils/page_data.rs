use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct PageData<T> {
    pub total: i64,
    pub list: Vec<T>,
}

impl<T> PageData<T>
where
    T: Serialize + for<'a> Deserialize<'a> + Clone,
{
    pub fn new(total: i64, list: Vec<T>) -> Self {
        PageData { total, list }
    }
}

pub fn query_order(
    params: &HashMap<String, String>,
    default_order_field: String,
    is_asc: bool,
) -> String {
    let cur_page = params
        .get("page")
        .unwrap_or(&"1".to_string())
        .parse::<i64>()
        .unwrap();
    let limit = params
        .get("limit")
        .unwrap_or(&"10".to_string())
        .parse::<i64>()
        .unwrap();

    let order_field = params
        .get("order_field")
        .unwrap_or(&"".to_string())
        .to_string();
    let order = params.get("order").unwrap_or(&"".to_string()).to_string();

    let order = if order.eq_ignore_ascii_case("asc") {
        "asc"
    } else if order.eq_ignore_ascii_case("desc") {
        "desc"
    } else if is_asc {
        "asc"
    } else {
        "desc"
    };

    let mut order_by = "".to_string();
    if !order_field.trim().is_empty() && !order.trim().is_empty() {
        order_by = format!(" order by {} {}", order_field, order);
    }

    if default_order_field.trim().is_empty() {
        return format!("{} limit {}, {}", order_by, (cur_page - 1) * limit, limit);
    }

    if is_asc {
        if order_by.is_empty() {
            order_by = format!(" order by {} asc", default_order_field);
        } else {
            order_by = format!("{} , {} asc", order_by, default_order_field);
        }
    } else if order_by.is_empty() {
        order_by = format!(" order by {} desc", default_order_field);
    } else {
        order_by = format!("{} , {} desc", order_by, default_order_field);
    }

    format!("{} limit {}, {}", order_by, (cur_page - 1) * limit, limit)
}
