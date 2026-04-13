use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::Settings;
use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub username: String,
    pub email: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(user_id: Uuid, username: String, email: String, expiration_hours: i64) -> Self {
        let now = Utc::now();
        Self {
            sub: user_id,
            username,
            email,
            iat: now.timestamp(),
            exp: (now + Duration::hours(expiration_hours)).timestamp(),
        }
    }
}

pub fn generate_token(
    user_id: Uuid,
    username: String,
    email: String,
    settings: &Settings,
) -> Result<String, AppError> {
    let claims = Claims::new(user_id, username, email, settings.jwt.expiration_hours);
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(settings.jwt.secret.as_bytes()),
    )
    .map_err(|e| AppError::InternalError(format!("Token generation failed: {}", e)))
}

pub fn decode_token(token: &str, settings: &Settings) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(settings.jwt.secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::AuthenticationError(format!("Invalid token: {}", e)))
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let settings = req
        .app_data::<actix_web::web::Data<Settings>>()
        .expect("Settings not found in app data");

    match decode_token(credentials.token(), settings) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(_) => Err((
            actix_web::error::ErrorUnauthorized("Invalid token"),
            req,
        )),
    }
}

/// Validator that checks for token in query parameters (for WebSocket connections)
pub async fn validator_with_query(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let settings = req
        .app_data::<actix_web::web::Data<Settings>>()
        .expect("Settings not found in app data");

    // Try to get token from Authorization header first
    let token = credentials.token();
    
    // If that fails, try to get from query parameter
    let token = if token.is_empty() {
        req.query_string()
            .split('&')
            .find_map(|pair| {
                let mut parts = pair.splitn(2, '=');
                let key = parts.next()?;
                let value = parts.next()?;
                if key == "token" {
                    Some(value.to_string())
                } else {
                    None
                }
            })
            .unwrap_or_default()
    } else {
        token.to_string()
    };

    if token.is_empty() {
        return Err((
            actix_web::error::ErrorUnauthorized("No token provided"),
            req,
        ));
    }

    match decode_token(&token, settings) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(_) => Err((
            actix_web::error::ErrorUnauthorized("Invalid token"),
            req,
        )),
    }
}
