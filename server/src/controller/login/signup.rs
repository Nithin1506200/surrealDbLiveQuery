use crate::{
    auth_service::get_token_from_user,
    common::error::{AppError, AppErrorType},
    controller::login::login::LoginResponse,
    db::users::{create, get_user_by_email},
};
use actix_web::{post, web, Result as ActixResult};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub name: String,
    pub password: String,
    pub merchant_id: String,
}
#[post("/signup")]
pub async fn signup_controller(
    body: web::Json<SignupRequest>,
) -> ActixResult<web::Json<LoginResponse>, AppError> {
    let response = get_user_by_email(&body.email).await;
    println!("---------------{:?}", response);
    if let Ok(_response) = response {
    } else {
        return Err(AppError {
            cause: Some("More users".to_string()),
            message: Some("user already present".to_string()),
            error_type: AppErrorType::BadRequest,
        });
    }
    let merchant: &str = &body.merchant_id;
    let user = create(&body.email, &body.password, merchant, &body.name).await?;
    let user = user[0].clone();
    let token = get_token_from_user(&user);
    Ok(web::Json(LoginResponse {
        token: token.clone(),
    }))
}
