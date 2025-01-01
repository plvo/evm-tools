use crate::{
    constant::{GWEI, WEI},
    model::Config,
};
use dotenv::dotenv;
use std::{
    io::{self, BufRead, Write},
    path::Path,
};

pub fn init() -> () {
    env_logger::Builder::new()
        .format(|buf, record| {
            let ts = buf.timestamp_millis();
            let level = match record.level() {
                log::Level::Error => "\x1b[31mERROR\x1b[0m", // Red
                log::Level::Warn => "\x1b[33mWARN\x1b[0m",   // Yellow
                log::Level::Info => "\x1b[32mINFO\x1b[0m",   // Green
                log::Level::Debug => "\x1b[34mDEBUG\x1b[0m", // Blue
                log::Level::Trace => "\x1b[35mTRACE\x1b[0m", // Purple
            };
            writeln!(
                buf,
                "[{} {} {}] {}",
                ts,
                level,
                record.target(),
                record.args()
            )
        })
        .filter(None, log::LevelFilter::Info)
        .init();

    dotenv().ok();
    println!(
        r#"
                                          __                       __          
                                         /  |                     /  |         
  ______  __     __ _____  ____         _$$ |_    ______   ______ $$ | _______ 
 /      \/  \   /  /     \/    \ ______/ $$   |  /      \ /      \$$ |/       |
/$$$$$$  $$  \ /$$/$$$$$$ $$$$  /      $$$$$$/  /$$$$$$  /$$$$$$  $$ /$$$$$$$/ 
$$    $$ |$$  /$$/ $$ | $$ | $$ $$$$$$/  $$ | __$$ |  $$ $$ |  $$ $$ $$      \ 
$$$$$$$$/  $$ $$/  $$ | $$ | $$ |        $$ |/  $$ \__$$ $$ \__$$ $$ |$$$$$$  |
$$       |  $$$/   $$ | $$ | $$ |        $$  $$/$$    $$/$$    $$/$$ /     $$/ 
$$$$$$$/    $/    $$/  $$/  $$/          $$$$/  $$$$$$/  $$$$$$/ $$/$$$$$$$/  
"#
    );
}

type RefPath = dyn AsRef<Path>;

pub fn is_file_exists(file: &RefPath) -> bool {
    file.as_ref().exists()
}

pub fn open_or_create_file(file: &RefPath) -> std::fs::File {
    match !is_file_exists(file) {
        true => std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(file)
            .unwrap(),
        false => std::fs::File::create(file).unwrap(),
    }
}

pub fn read_lines(filename: &RefPath) -> io::Result<Vec<String>>{
    let file = std::fs::File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    reader.lines().collect()
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

pub fn wei_to_gwei(wei: f64) -> f64 {
    wei as f64 / GWEI
}

pub fn wei_to_eth(wei: f64) -> f64 {
    wei as f64 / WEI
}

pub fn get_rpc_from_config(config_data: Option<Config>, network: &String) -> Option<String> {
    let config: Config = config_data.unwrap_or_else(|| read_config_file());

    for net in config.network.iter() {
        if net.contains_key(network) {
            return Some(net.get(network).unwrap().to_string());
        }
    }

    None
}

pub fn log_and_panic<T, E: std::fmt::Debug>(result: Result<T, E>, msg: &str) -> T {
    result.unwrap_or_else(|e| {
        log::error!("{}: {:?}", msg, e);
        panic!("{}", msg);
    })
}