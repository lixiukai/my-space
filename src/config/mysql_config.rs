use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn connect_mysql(mysql_url: String) -> DatabaseConnection {
    // 连接
    let mut opt = ConnectOptions::new(mysql_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false)
    // .sqlx_logging_level(log::LevelFilter::Info)
    .set_schema_search_path("db_space");
    Database::connect(opt).await.unwrap()
}