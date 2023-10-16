use crate::{
    auth_service::get_token,
    common::{
        self,
        error::{AppError, AppErrorType},
        merchant::Merchants,
    },
    db::{self, users::get_user_by_email},
};
use actix_web::{get, post, web, Result as ActixResult};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    token: String,
}
#[post("/login")]
pub async fn login_controller(
    body: web::Json<LoginRequest>,
) -> ActixResult<web::Json<LoginResponse>, AppError> {
    let users = db::users::validate(&body.email, &body.password).await?;
    if users.0 {
        let user = users.1[0].clone();
        let token = get_token(&user);
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
