pub mod common;
pub mod config;
pub mod modules;

const TOKEN_HEADER: &str = "token";

lazy_static::lazy_static! {
    pub static ref CONFIG: config::Config = config::load_config();
}
