use rocket::http::{ContentType, Status};
use rocket::{Request, Response};
use rocket::response::{Responder, Result};
use rocket::serde::Serialize;
use serde_json::to_string;

#[derive(Serialize)]
pub struct CustomResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: Vec<T>
}

// 将对象转换为http响应
impl<'r, 'o:'r, T: Serialize> Responder<'r, 'o> for CustomResponse<T> {
    fn respond_to(self, request: &'r Request<'_>) -> Result<'o> {
        let json_data = to_string(&self).unwrap(); // 序列化数据为JSON字符串
        Response::build_from(json_data.respond_to(request)?)
            .status(Status::from_code(self.code).unwrap()) // 设置响应状态码
            .header(ContentType::JSON)
            .ok()
    }
}

impl<T> CustomResponse<T> {
    pub fn build(code: u16, message: String, data: Vec<T>) -> CustomResponse<T> {
        CustomResponse {
            code,
            message,
            data
        }
    }
}