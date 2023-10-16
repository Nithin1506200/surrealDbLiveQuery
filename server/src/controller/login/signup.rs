use actix_web::{post, web, Result as ActixResult};
use serde::{Deserialize, Serialize};

use crate::{
    common::{
        error::{AppError, AppErrorType},
        merchant::Merchants,
    },
    controller::login::login::LoginResponse,
    db::users::get_user_by_email,
};

#[derive(Serialize, Deserialize)]
pub struct SignupRequest {
    email: String,
    password: String,
    merchant_id: String,
}
#[post("/signup")]
pub async fn signup(
    body: web::Json<SignupRequest>,
) -> ActixResult<web::Json<LoginResponse>, AppError> {
    let response = get_user_by_email(&body.email).await?;

    if response.len() > 0 {
        return Err(AppError {
            cause: Some("More users".to_string()),
            message: Some("user already present".to_string()),
            error_type: AppErrorType::BadRequest,
        });
    }
    let merchant: Result<Merchants, _> = serde_json::from_str(&body.merchant_id);
    if let Err(_) = merchant {
        return Err(AppError {
            cause: Some("Merchant not registered".to_string()),
            message: Some("Merchant not registered".to_string()),
            error_type: AppErrorType::BadRequest,
        });
    }

    todo!()
}
