use crate::model::Config;

pub fn is_file_exists(file: &String) -> bool {
    std::path::Path::new(file).exists()
}

pub fn open_or_create_file(file: &String) -> std::fs::File {
    match !is_file_exists(file) {
        true => std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(file)
            .unwrap(),
        false => std::fs::File::create(file).unwrap(),
    }
}

pub fn timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

pub fn read_config_file() -> Config {
    let config_data = std::fs::read_to_string("./config.json").expect("Unable to read config file");
    serde_json::from_str(&config_data).expect("Unable to parse config file")
}

pub fn get_rpc_from_config(config_data:Option<Config>, network: &String) -> Option<String> {
    let config = match config_data {
        Some(config) => config,
        None => read_config_file()
    };

    for net in config.network.iter() {
        if net.contains_key(network) {
            return Some(net.get(network).unwrap().to_string());
        }
    }

    None
}
