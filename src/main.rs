use axum::{Router, http::StatusCode, response::IntoResponse};
use backend::AppState;

pub async fn handle_home() -> impl IntoResponse {
    (StatusCode::OK, "Welcome to URL SHORTNER API")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Sled DB Initialisation
    let db = sled::open("db").expect("Can't connect to the db");
    let url_tree = db.open_tree("url")?;

    // Axum Router Initailisation

    let state = AppState{
        url_db: url_tree
    };

    let app = Router::new()
        .route("/", axum::routing::get(handle_home))
        .with_state(state);

    // TCP Connection Initialisation 
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("Running on http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
