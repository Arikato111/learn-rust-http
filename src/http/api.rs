use super::Response;
use serde_json::json;

pub fn index() -> Response {
    let info = json!({
        "name": "nawasan wisitsingkhon",
    });
    Response::new(super::HttpStatus::OK, Some(info.to_string()))
}

pub fn hello() -> Response {
    let res_data = json!({
        "message" : "hello"
    });
    Response::new(super::HttpStatus::OK, Some(res_data.to_string()))
}
