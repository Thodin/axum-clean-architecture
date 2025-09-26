use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

use crate::{
    adapters::http::app_state::AppState, app_error::AppResult, use_cases::user::UserUseCases,
};

pub fn router() -> Router<AppState> {
    Router::new().route("/register", post(register))
}

#[derive(Debug, Clone, Deserialize)]
struct RegisterPayload {
    username: String,
    email: String,
    password: SecretString,
}

#[derive(Debug, Clone, Serialize)]
struct RegisterResponse {
    success: bool,
}

/// Creates a new user based on the submitted credentials.
#[instrument(skip(user_use_cases))]
async fn register(
    State(user_use_cases): State<Arc<UserUseCases>>,
    Json(payload): Json<RegisterPayload>,
) -> AppResult<impl IntoResponse> {
    info!("Register user called");
    user_use_cases
        .add(&payload.username, &payload.email, &payload.password)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterResponse { success: true }),
    ))
}
