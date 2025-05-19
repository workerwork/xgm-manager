#[cfg_attr(mobile, tauri::mobile_entry_point)]
mod commands;
mod models;
mod services;
mod utils;

use crate::commands::{
    auth::login,
    device::{
        delete_device, get_device_config, list_devices, refresh_device_config, update_device_config,
    },
    log::{clear_log_list, get_log_list},
    network::update_network_config,
    package::{get_package_detail, get_package_list, upload_package},
};
use crate::utils::{
    jupyter::start_jupyter_notebook, logger::setup_log_plugin,
    udp_listener::start_udp_listener,
};
use dashmap::DashMap;
use std::sync::Arc;
use std::thread;

pub fn run() {
    let devices = Arc::new(DashMap::new());

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(devices.clone()) // 使用 .manage 来管理状态
        .setup(move |app| {
            // 配置日志插件
            setup_log_plugin(app)?;

            // 启动 jupyter notebook 服务器
            thread::spawn(move || {
                start_jupyter_notebook();
            });

            // 使用线程启动 UDP 监听器
            thread::spawn(move || {
                start_udp_listener(devices.clone());
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            login,
            list_devices,
            delete_device,
            get_device_config,
            update_device_config,
            refresh_device_config,
            get_package_list,
            get_package_detail,
            upload_package,
            update_network_config,
            get_log_list,
            clear_log_list,
        ]) // 注册命令
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
