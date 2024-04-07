use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use crate::utils::custom_response::CustomResponse;
use crate::services::impls::user_service_impl::UserServiceImpl;
use crate::services::user_service::UserService;
use crate::utils::typing::{DB, RegisterParam};

// 用户注册
#[post("/register", data="<param>")]
pub async fn register_handler(state: &State<DB>, param: Json<RegisterParam>)
    -> CustomResponse<()> {
    let db = Box::new(state.mysql.clone());
    let service = Box::new(UserServiceImpl::build(db));
    match service.register_service(Box::new(param.into_inner())).await {
        Ok(value) => CustomResponse::build(Status::Ok.code, value, vec![]),
        Err(err) => {
            let (code, msg) = err;
            CustomResponse::build(code, msg, vec![])
        }
    }
}

// 用户登录
#[post("/login")]
pub async fn login_handler()
    -> CustomResponse<()> {
    CustomResponse::build(Status::Ok.code, "登录成功".to_string(), vec![])
}