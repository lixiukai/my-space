use rocket::futures::io;
use crate::utils::typing::{LoginParam, RegisterParam};

pub trait UserService {
    // 账号注册
    async fn register_service(&self, param: Box<RegisterParam>)
        -> Result<String, (u16, String)>;

    // 账号登录
    async fn login_service(&self, param: &LoginParam)
        -> Result<String, io::Error>;
}