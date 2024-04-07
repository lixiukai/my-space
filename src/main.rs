mod handlers;
mod utils;
mod config;
mod repositories;
mod services;
mod models;


use handlers::user_handler;
use utils::typing::DB;
use crate::handlers::errors_handler::{
    bad_request,
    forbidden,
    internal_server_error,
    no_authorization,
    not_found
};

#[macro_use] extern crate rocket;


#[launch]
async fn rocket() -> _ {
    let mysql_addr = config::read_mysql_config_file(); // 读取配置文件
    let mysql = config::mysql_config::connect_mysql(mysql_addr).await; // 连接数据库

    rocket::build()
        .manage(DB { mysql }) // 共享数据库
        .mount("/api/user", routes![user_handler::register_handler]) // 挂载路由和处理函数
        .mount("/api/user", routes![user_handler::login_handler])
        .register("/", catchers![ // 注册错误监听
            bad_request,
            no_authorization,
            forbidden,
            not_found,
            internal_server_error
        ])

}