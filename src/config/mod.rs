pub mod mysql_config;

// 读取mysql配置文件，生成连接地址
pub fn read_mysql_config_file() -> String {
    let toml_str = include_str!("configuration.toml");
    let config = toml::from_str::<toml::Value>(toml_str).unwrap();
    // 读取 mysql 配置
    let mysql = config.get("mysql").unwrap();
    let ip = mysql.get("ip").unwrap().as_str().unwrap();
    let port = mysql.get("port").unwrap().as_str().unwrap();
    let username = mysql.get("username").unwrap().as_str().unwrap();
    let password = mysql.get("password").unwrap().as_str().unwrap();
    let database = mysql.get("database").unwrap().as_str().unwrap();
    format!("mysql://{}:{}@{}:{}/{}", username, password, ip, port, database)
}

// pub fn read_redis_config_file() -> String {
//     let toml_str = include_str!("configuration.toml");
//     let config = toml::from_str::<toml::Value>(toml_str).unwrap();
//     // 读取 mysql 配置
//     let redis = config.get("redis").unwrap();
//     let ip = redis.get("ip").unwrap().as_str().unwrap();
//     let port = redis.get("port").unwrap().as_str().unwrap();
//     let auth = redis.get("auth").unwrap().as_str().unwrap();
//     format!("redis://:{}@{}:{}", auth, ip, port)
// }