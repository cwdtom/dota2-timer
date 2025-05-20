use std::fs;
use serde_json;

// default config name
const DEFAULT_CONFIG: &str = "default";

/// notice node config
#[derive(serde::Deserialize)]
pub struct NoticeConfig {
    // period seconds
    pub period: i32,
    // content
    pub text: String,
    // start timestamp
    pub start_time: i32,
    // end timestamp
    pub end_time: i32,
    // repeat count
    pub repeat_count: i32,
    // early notice time seconds
    pub early_notice_time: i32,
}

/// get config list
pub fn get_text_list(dir: &str) -> Vec<String> {
    let mut list:Vec<String> = fs::read_dir(dir)
        .unwrap()
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path().file_name().and_then(|name| {
                    name.to_str().map(|s| {
                        let parts: Vec<&str> = s.split(".").collect();
                        return parts[0].to_string();
                    })
                })
            })
        })
        .collect();

    // sort list and ensure default config is first
    list.retain(|x| x != DEFAULT_CONFIG);
    list.sort();
    list.insert(0, DEFAULT_CONFIG.to_string());

    return list;
}

/// get config list
pub fn get_notice_config_list(config_name: Option<String>) -> Vec<NoticeConfig> {
    // read config file
    let config_path = match config_name {
        Some(name) => format!("config/{}.json", name),
        None => format!("config/{}.json", DEFAULT_CONFIG),
    };
    let content = fs::read_to_string(config_path).unwrap();

    // json to struct
    let notice_node_list: Vec<NoticeConfig> = serde_json::from_str(&content).unwrap();
    notice_node_list
}
