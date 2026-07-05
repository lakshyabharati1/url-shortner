use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use crate::{AppState, url_management::{is_absolute_url, new_url::new_url}};

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

    if !is_absolute_url(&json.url).await {
        return (
            StatusCode::BAD_REQUEST, 
            Json(("reason", "url is in the wrong format, make sure it is formatted as http://www.example.com or https://www.example.com"))
            ).into_response();
    }

    
    let response = new_url(&json.url.as_str(), state).await;

    match response {
        Ok(x) => (StatusCode::OK, Json(CreateNewURLRes { shortened_key: x })).into_response(),
        Err(_x) => StatusCode::BAD_REQUEST.into_response(),
    }
}
