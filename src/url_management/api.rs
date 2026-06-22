use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{AppState, url_management::new_url::new_url};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNewURLInput {
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNewURLRes {
    shortened_key: String,
}
pub async fn new_url_api(
    State(state): State<AppState>,
    json: Json<CreateNewURLInput>,
) -> impl IntoResponse {
    let response = new_url(&json.url.as_str(), state).await;

    match response {
        Ok(x) => (StatusCode::OK, Json(CreateNewURLRes { shortened_key: x })).into_response(),
        Err(_x) => StatusCode::BAD_REQUEST.into_response(),
    }
}
