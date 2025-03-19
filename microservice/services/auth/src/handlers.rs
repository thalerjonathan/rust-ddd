use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
};
use log::info;
use microservices_shared::token::{AccessToken, Tokens};
use opentelemetry::trace::Tracer;
use redis::Commands;
use serde::{Deserialize, Serialize};
use shared::app_error::AppError;

use crate::AppState;

static ACCESS_TOKEN_COOKIE: &str = "rust-ddd-access";

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginDTO {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthStatus {
    LoggedIn,
    NotLoggedIn,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthStatusDTO {
    status: AuthStatus,
}

pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(login_info): Json<LoginDTO>,
) -> Result<impl IntoResponse, AppError> {
    info!("Login {:?}", login_info);
    let _span = state.tracer.start("login");

    let access_token_lookup = extract_access_token_from_cookie(&headers);

    match access_token_lookup {
        Some(acces_token_encoded) => {
            info!("Access token already present in cookie");

            // TODO: might fail
            let access_token: AccessToken = acces_token_encoded
                .try_into()
                .map_err(|e: String| AppError::from_error(e.as_str()))?;

            info!("access_token: {:?}", access_token);

            // TODO: introspect access token for validity either via JWK or introspect endpoint
            // TODO: look up in Redis

            let headers = HeaderMap::new();
            Ok((headers, "Logged in, cookie set!"))
        }
        None => {
            info!("Access token not found in cookies, try to log in");

            let idp_token = state
                .token_manager
                .request_idp_tokens_via_credentials(&login_info.username, &login_info.password)
                .await
                .map_err(|e| AppError::from_error(&e.to_string()))?;

            let tokens: Tokens = idp_token
                .clone()
                .try_into()
                .map_err(|e: String| AppError::from_error(e.as_str()))?;

            let serialized_tokens = serde_json::to_string(&tokens.clone())
                .map_err(|e| AppError::from_error(&e.to_string()))?;
            let mut redis_conn_mut = state.redis_conn.lock().await;

            let redis_result: Result<(), redis::RedisError> =
                redis_conn_mut.set(&tokens.identity.sub.clone(), serialized_tokens);
            redis_result.map_err(|e| AppError::from_error(&e.to_string()))?;

            // NOTE: we construct `Set-Cookie` header manually because unable to get tower-cookies to work
            let cookie_value = format!(
                "{}={}; Path=/; HttpOnly; SameSite=Lax",
                ACCESS_TOKEN_COOKIE, idp_token.access_token
            );

            let mut headers = HeaderMap::new();
            headers.insert("Set-Cookie", HeaderValue::from_str(&cookie_value).unwrap());

            Ok((headers, "Logged in, access token cookie set!"))
        }
    }
}

pub async fn status_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<AuthStatusDTO>, AppError> {
    info!("Status");
    let _span = state.tracer.start("status");

    if let Some(access_token_encoded) = extract_access_token_from_cookie(&headers) {
        info!("Access token found in cookie");
        let access_token_decoded: Result<AccessToken, String> = access_token_encoded.try_into();
        match access_token_decoded {
            Ok(access_token) => {
                info!("access_token: {:?}", access_token);

                // TODO: introspect access token for validity either via JWK or introspect endpoint
                // TODO: look up in Redis

                return Ok(Json(AuthStatusDTO {
                    status: AuthStatus::LoggedIn,
                }));
            }
            Err(err) => {
                info!("Failed to decode access token in cookie: {:?}", err)
            }
        }
    }

    Ok(Json(AuthStatusDTO {
        status: AuthStatus::NotLoggedIn,
    }))
}

fn extract_access_token_from_cookie(headers: &HeaderMap) -> Option<String> {
    let cookie_header = headers.get("cookie")?;
    let cookie_str = cookie_header.to_str().ok()?.to_string();
    let cookies: Vec<&str> = cookie_str.split("; ").collect();

    for cookie in cookies.iter() {
        let cookie_split: Vec<&str> = cookie.split("=").collect();
        if cookie_split.len() == 2 {
            if cookie_split[0] == ACCESS_TOKEN_COOKIE {
                return Some(cookie_split[1].to_string());
            }
        }
    }

    None
}
