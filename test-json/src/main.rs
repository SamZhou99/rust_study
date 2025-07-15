use serde_json::Value;
use std::fs;

fn main() {
    // JSON 数据
    let json_str = fs::read_to_string("config.json").unwrap();
    let config_data: Value = serde_json::from_str(&json_str).unwrap();
    println!(
        "解析后的结构体: \n{:?}, \n{:?}, \n{:?}, \n{:?}",
        config_data["ConfigAir"]["SocketServer"]["IP"],
        config_data["ConfigAir"]["SocketServer"]["Port"],
        config_data["ConfigAir"]["Type"]["Data"][0],
        config_data["ConfigAir"]["Template"]["PicPk"]["Data"][0],
    );
    // pt(&config);
    // // 将 Rust 结构体转换为 JSON
    // let serialized = serde_json::to_string(&config).unwrap();
    // println!("序列化后的 JSON: {}", serialized);
    // pt(&serialized);
}
