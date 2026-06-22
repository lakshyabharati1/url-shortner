#[derive(Clone)]
pub struct AppState {
    pub url_db: sled::Tree
}

pub mod url_management;
