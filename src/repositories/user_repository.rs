use std::time::{ SystemTime, UNIX_EPOCH};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use crate::models::prelude::TbUser;
use crate::models::tb_user;
use crate::utils::typing::RegisterParam;

pub struct UserRepository {
    mysql: Box<DatabaseConnection>
}

impl UserRepository {
    pub fn build(mysql: Box<DatabaseConnection>) -> Self {
        UserRepository {
            mysql
        }
    }

    // 检查用户名是否重复
    pub async fn check_user_has_exist(&self, username: String) -> bool {
        let db = *self.mysql.clone();
        let result =
            TbUser::find().filter(tb_user::Column::Username.eq(username)).one(&db).await;
        if let Ok(Some(_)) = result {
            return true;
        }
        false
    }

    // 新增数据
    pub async fn create_user(&self, data: Box<RegisterParam>) -> Result<(), DbErr> {
        let time =
            SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis() as i64;
        let user = tb_user::Model {
            id: time.to_string(),
            username: data.username,
            password: data.password,
            problem: data.problem,
            answer: data.answer,
            nickname: data.nickname,
            avatar: Some(data.avatar),
            role: Some(2),
            status: Some(0),
            create_time: time,
            update_time: time,
            deleted_at: None,
        };
        let db = *self.mysql.clone();
        let active_model = tb_user::ActiveModel::from(user);
        active_model.insert(&db).await?;
        Ok(())
    }
}