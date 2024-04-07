use sea_orm::DatabaseConnection;
use serde::{Deserialize};

pub struct DB {
    pub mysql: DatabaseConnection
}

#[derive(Deserialize, Debug, Clone)]
pub struct RegisterParam {
    pub username: String,
    pub password: String,
    pub problem: String,
    pub answer: String,
    pub nickname: String,
    pub avatar: String,
}

pub struct LoginParam {
    username: String,
    password: String,
}