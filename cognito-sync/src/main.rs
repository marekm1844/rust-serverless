use lambda_http::{lambda, Body, IntoResponse, Request, Response};
use lambda_runtime::{error::HandlerError, Context};
use serde::{Deserialize, Serialize};

use std::error::Error;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct User {
    username: String,
    email: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(router);
    Ok(())
}

fn router(req: Request, c: Context) -> Result<impl IntoResponse, HandlerError> {
    match req.method().as_str() {
        "GET" => get_user(req, c),
        _ => {
            let mut resp = Response::default();
            *resp.status_mut() = http::StatusCode::METHOD_NOT_ALLOWED;
            Ok(resp)
        }
    }
}

fn get_user(_req: Request, _c: Context) -> Result<Response<Body>, HandlerError> {
    let user = User {
        username: "Marek".to_owned(),
        email: "test@test.com".to_owned(),
    };

    Ok(serde_json::json!(user).into_response())
}
