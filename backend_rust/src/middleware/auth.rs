use actix_web::{dev::Payload, FromRequest, HttpMessage, HttpRequest};
use std::future::{ready, Ready};

use crate::errors::AppError;
use crate::utils::auth::Claims;

pub struct AuthenticatedUser {
    pub user_id: uuid::Uuid,
    pub username: String,
    pub email: String,
}

impl From<Claims> for AuthenticatedUser {
    fn from(claims: Claims) -> Self {
        Self {
            user_id: claims.sub,
            username: claims.username,
            email: claims.email,
        }
    }
}

impl FromRequest for AuthenticatedUser {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let extensions = req.extensions();
        
        match extensions.get::<Claims>() {
            Some(claims) => ready(Ok(AuthenticatedUser::from(claims.clone()))),

            None => ready(Err(AppError::AuthenticationError(
                "User not authenticated".to_string()
            ))),
        }
    }
}
