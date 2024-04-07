//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "tb_user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub username: String,
    pub password: String,
    pub problem: String,
    pub answer: String,
    pub nickname: String,
    pub avatar: Option<String>,
    pub role: Option<i32>,
    pub status: Option<i32>,
    pub create_time: i64,
    pub update_time: i64,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
