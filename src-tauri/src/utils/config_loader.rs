use once_cell::sync::Lazy;
use std::sync::Mutex;
use config::{Config, ConfigError, File, Environment};
use crate::models::config::AppConfig;

pub fn load_config() -> Result<AppConfig, ConfigError> {
    let builder = Config::builder()
        .add_source(File::with_name("config/default")) // 加载默认配置
        .add_source(Environment::with_prefix("APP").separator("__")); // 加载环境变量

    builder.build()?.try_deserialize()
}


pub fn reload_config() -> Result<(), ConfigError> {
    let new_config = load_config()?;
    let mut cfg = CONFIG.lock().unwrap();
    *cfg = new_config;
    Ok(())
}


// ✅ 全局配置只加载一次，后续可随时访问
pub static CONFIG: Lazy<Mutex<AppConfig>> = Lazy::new(|| {
    let config = load_config().expect("Failed to load config");
    Mutex::new(config)
});

pub fn get_config() -> AppConfig {
    CONFIG.lock().unwrap().clone()
}
