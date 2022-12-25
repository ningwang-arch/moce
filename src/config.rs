use log::info;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const CONFIG_FILE: &str = "config.toml";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MysqlConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl MysqlConfig {
    pub fn new(host: String, port: u16, user: String, password: String, database: String) -> Self {
        MysqlConfig {
            host,
            port,
            user,
            password,
            database,
        }
    }
}

impl Default for MysqlConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3306,
            user: "root".to_string(),
            password: "root".to_string(),
            database: "db".to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RocketConfig {
    pub address: String,
    pub port: u16,
    pub workers: usize,
    pub log_level: String,
    pub ident: String,
}

impl RocketConfig {
    pub fn new(
        address: String,
        port: u16,
        workers: usize,
        log_level: String,
        ident: String,
    ) -> Self {
        RocketConfig {
            address,
            port,
            workers,
            log_level,
            ident,
        }
    }
}

impl Default for RocketConfig {
    fn default() -> Self {
        Self {
            address: "127.0.0.1".to_string(),
            port: 8000,
            workers: 4,
            log_level: "normal".to_string(),
            ident: "Rocket".to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub password: String,
    pub database: u8,
}

impl RedisConfig {
    pub fn new(host: String, port: u16, password: String, database: u8) -> Self {
        RedisConfig {
            host,
            port,
            password,
            database,
        }
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 6379,
            password: "".to_string(),
            database: 0,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Config {
    pub mysql: MysqlConfig,
    pub rocket: RocketConfig,
    pub redis: RedisConfig,
}

impl Config {
    pub fn new(mysql: MysqlConfig, rocket: RocketConfig, redis: RedisConfig) -> Self {
        Config {
            mysql,
            rocket,
            redis,
        }
    }
}

pub fn config_path() -> PathBuf {
    let mut path = config_path_dir();
    path.push(CONFIG_FILE);
    path
}

#[cfg(unix)]
pub fn config_path_dir() -> PathBuf {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("moce").unwrap();
    xdg_dirs.get_config_home()
}

#[cfg(windows)]
pub fn config_path_dir() -> PathBuf {
    let local_app_data = std::env::var("LOCALAPPDATA").unwrap();
    PathBuf::from(local_app_data).join("moce")
}

pub fn save_config(config: &Config) -> std::io::Result<()> {
    let config_path = config_path();
    info!("Saving config to {:?}", config_path);
    std::fs::create_dir_all(config_path.parent().unwrap())?;
    let config_str = toml::to_string_pretty(config).unwrap();
    std::fs::write(config_path, config_str)
}

pub fn load_config() -> Config {
    let config_path = config_path();
    info!("Loading config from {:?}", config_path);
    let config_str = std::fs::read_to_string(config_path);

    if let Ok(config_str) = config_str {
        match toml::from_str(&config_str) {
            Ok(config) => config,
            Err(e) => {
                info!("Error parsing config: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        info!("No config file found, using default config and saving to disk");
        let config = Config::default();
        save_config(&config).unwrap();
        config
    }
}
