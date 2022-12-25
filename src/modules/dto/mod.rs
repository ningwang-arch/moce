pub mod login_dto;
pub mod password_dto;
pub mod sys_dept_dto;
pub mod sys_dict_type_dto;
pub mod sys_menu_dto;
pub mod sys_params_dto;
pub mod sys_role_dto;
pub mod sys_user_dto;

pub trait TreeNode
where
    Self: Sized,
{
    fn get_id(&self) -> i64;
    fn get_pid(&self) -> i64;
    fn add_child(&mut self, child: Self);

    fn get_children(&self) -> &Vec<Self>;
    fn get_children_mut(&mut self) -> &mut Vec<Self>;
}

pub fn build<T: TreeNode + Clone>(root: &mut T, list: &mut Vec<T>) {
    if list.is_empty() {
        return;
    }

    let mut i = 0;
    while i < list.len() {
        let item = list.get(i).unwrap();
        if item.get_pid() == root.get_id() {
            root.add_child(item.clone());
            list.remove(i);
        } else {
            i += 1;
        }
    }

    for item in root.get_children_mut() {
        build(item, list);
    }
}
