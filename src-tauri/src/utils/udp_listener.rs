use crate::models::device::{DeviceInfo, DevicePayload};
use dashmap::DashMap;
use log::{error, info};
use std::net::UdpSocket;
use std::sync::Arc;

// UDP 监听器启动函数，改成异步函数？
pub fn start_udp_listener(devices: Arc<DashMap<String, DeviceInfo>>) {
    let sock = UdpSocket::bind("0.0.0.0:5010").expect("无法绑定UDP端口");
    info!("正在监听设备广播...");

    let mut buf = [0u8; 1024];
    while let Ok((size, addr)) = sock.recv_from(&mut buf) {
        let ip = addr.ip().to_string();
        let payload_str = match std::str::from_utf8(&buf[..size]) {
            Ok(s) => s,
            Err(e) => {
                error!("数据解析失败: {:?}", e);
                continue;
            }
        };

        match serde_json::from_str::<DevicePayload>(payload_str) {
            Ok(payload) => {
                let info = DeviceInfo::from_payload(payload, ip);
                info!("✅ 收到设备信息: {:?}", info);

                // 使用 Arc<DashMap> 的线程安全方式插入设备信息
                devices.insert(info.sn.clone(), info);
            }
            Err(e) => {
                error!("无效的设备数据: {}", e);
            }
        }
    }
}
