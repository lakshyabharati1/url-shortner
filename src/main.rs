use std::sync::Arc;

use axum::{
    Router, extract::{Path, State}, http::StatusCode, response::{IntoResponse, Redirect}, routing::{get, post}
};
use backend::{AppState, admin::api::get_entries_api, get_last_url, url_management::{api::new_url_api, route::router}};
use tokio::sync::Mutex;

pub async fn home() -> impl IntoResponse {
    (StatusCode::OK, "URL_SHORTNER_API").into_response()
}

pub async fn handle_routing(Path(params): Path<String>, State(state): State<AppState>) -> impl IntoResponse {
    
    if params.as_str() == "" {
        return (StatusCode::OK, "Welcome to URL SHORTNER API").into_response();
    }
    
    let routing_path = match router(params, state).await{
        Ok(x) => x,
        Err(_x) => None
    };

    match routing_path {
        Some(path) => {
            if path.starts_with("https://") || path.starts_with("http://") {
                return Redirect::permanent(&path).into_response();
            }

            else {
                return Redirect::permanent(&format!("http://{}", path)).into_response();
            }
        }
        None => {
            return (StatusCode::NOT_FOUND, "URL NOT FOUND").into_response();
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Sled DB Initialisation
    let db = sled::open("db").expect("Can't connect to the db");
    let url_tree = db.open_tree("url")?;

    // Axum Router Initailisation
    let last_url = match get_last_url(url_tree.clone()).await {
        Ok(x) => x,
        Err(x) => {
            eprintln!("Error: {}", x);
            panic!("");
        }
    };
    
    println!("Last Shortened URL was with key: {}", last_url);
    
    let state = AppState {
        url_db: url_tree,
        admin_verification_code: String::from("1234"),
        last_url: Arc::new(Mutex::new(last_url)),
    };

    let app = Router::new()
        .route("/", get(home))
        .route("/{params}", axum::routing::get(handle_routing))
        .route("/new-url", post(new_url_api))
        .route("/admin/get_entries", get(get_entries_api))
        .with_state(state);

    // TCP Connection Initialisation
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("Running on http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
