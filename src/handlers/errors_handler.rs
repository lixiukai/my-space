use rocket::Request;
use crate::utils::custom_response::CustomResponse;

#[catch(400)]
pub fn bad_request(_: &Request) -> CustomResponse<()> {
    CustomResponse {
        code: 400,
        message: String::from("请求的参数/语法有误"),
        data: vec![]
    }
}

#[catch(401)]
pub fn no_authorization(_: &Request) -> CustomResponse<()> {
    CustomResponse {
        code: 401,
        message: String::from("需要进行身份认证"),
        data: vec![]
    }
}

#[catch(403)]
pub fn forbidden(_: &Request) -> CustomResponse<()> {
    CustomResponse {
        code: 403,
        message: String::from("没有权限执行此操作"),
        data: vec![]
    }
}

#[catch(404)]
pub fn not_found(_: &Request) -> CustomResponse<()> {
    CustomResponse {
        code: 404,
        message: String::from("请求的资源不存在"),
        data: vec![]
    }
}


#[catch(500)]
pub fn internal_server_error(_: &Request) -> CustomResponse<()> {
    CustomResponse {
        code: 500,
        message: String::from("服务器内部错误"),
        data: vec![]
    }
}