use axum::{
    Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use backend::{AppState, admin::api::get_entries_api, url_management::api::new_url_api};

pub async fn handle_home() -> impl IntoResponse {
    (StatusCode::OK, "Welcome to URL SHORTNER API")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Sled DB Initialisation
    let db = sled::open("db").expect("Can't connect to the db");
    let url_tree = db.open_tree("url")?;

    // Axum Router Initailisation

    let state = AppState {
        url_db: url_tree,
        admin_verification_code: String::from("1234"),
    };

    let app = Router::new()
        .route("/", axum::routing::get(handle_home))
        .route("/new-url", post(new_url_api))
        .route("/admin/get_entries", get(get_entries_api))
        .with_state(state);

    // TCP Connection Initialisation
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("Running on http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
