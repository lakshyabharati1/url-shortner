use std::error::Error;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Serialize, Deserialize)]
pub struct GetEntriesInput {
    pub admin_verification_code: String,
    pub search: Option<String>,
    pub count: u32,
}

pub async fn get_entries(
    json: GetEntriesInput,
    state: AppState,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let max_count = &json.count;
    let seach_string = match &json.search {
        Some(x) => x.as_bytes(),
        None => b"",
    };

    let tree = state.url_db;

    let iter = tree.range(seach_string..).filter_map(|x| x.ok());
    let mut res: Vec<(String, String)> = Vec::with_capacity(*max_count as usize);

    for (k, v) in iter.take(*max_count as usize) {
        res.push((
            String::from_utf8(k.to_vec())?,
            String::from_utf8(v.to_vec())?,
        ));
    }

    Ok(res)
}

pub struct GetEntriesResponse {
    pub count: u32,
    pub entries: Json<Vec<(String, String)>>,
}

pub async fn get_entries_api(
    State(state): State<AppState>,
    Query(json): Query<GetEntriesInput>,
) -> impl IntoResponse {
    if json.admin_verification_code != state.admin_verification_code {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    let res = get_entries(json, state).await;

    match res {
        Ok(x) => (StatusCode::OK, Json(x)).into_response(),
        Err(_x) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
