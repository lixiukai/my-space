use std::io;
use sea_orm::DatabaseConnection;
use crate::repositories::user_repository::UserRepository;
use crate::services::user_service::UserService;
use crate::utils::typing::{LoginParam, RegisterParam};

pub struct UserServiceImpl {
    repository: UserRepository
}

impl UserServiceImpl {
    pub fn build(db: Box<DatabaseConnection>) -> Self {
        UserServiceImpl {
            repository: UserRepository::build(db)
        }
    }
}

impl UserService for UserServiceImpl {
    async fn register_service(&self, param: Box<RegisterParam>)
        -> Result<String, (u16, String)> {
        // 1.查询用户名是否存在
        if self.repository.check_user_has_exist(param.clone().username).await {
            return Err((409, "用户名已存在".to_string()));
        }
        // 2.新增数据
        match self.repository.create_user(param.clone()).await {
            Ok(_) => Ok(String::from("注册用户成功")),
            Err(_) => Err((500, "注册用户失败".to_string()))
        }
    }

    async fn login_service(&self, _: &LoginParam)
        -> Result<String, io::Error> {
        todo!()
    }
}