use rocket::fairing::AdHoc;

pub mod login_controller;
pub mod sys_dept_controller;
pub mod sys_dict_type_controller;
pub mod sys_menu_controller;
pub mod sys_params_controller;
pub mod sys_role_controller;
pub mod sys_user_controller;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Controller", |rocket| async {
        rocket
            .attach(login_controller::stage())
            .attach(sys_dict_type_controller::stage())
            .attach(sys_menu_controller::stage())
            .attach(sys_user_controller::stage())
            .attach(sys_dept_controller::stage())
            .attach(sys_role_controller::stage())
    })
}
