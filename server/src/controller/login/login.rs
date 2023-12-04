use crate::{
    auth_service::get_token_from_user,
    common::error::{AppError, AppErrorType},
    db::{self},
};
use actix_web::{post, web, Result as ActixResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}
#[post("/login")]
pub async fn login_controller(
    body: web::Json<LoginRequest>,
) -> ActixResult<web::Json<LoginResponse>, AppError> {
    let users = db::users::validate(&body.email, &body.password).await?;
    if users.0 {
        let user = users.1[0].clone();
        let token = get_token_from_user(&user);
        Ok(web::Json(LoginResponse {
            token: token.clone(),
        }))
    } else {
        Err(AppError {
            message: Some("Invalid email or password".to_string()),
            cause: None,
            error_type: AppErrorType::NotFound,
        })
    }
}
