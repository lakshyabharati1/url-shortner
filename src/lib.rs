#[derive(Clone)]
pub struct AppState {
    pub url_db: sled::Tree,
    pub admin_verification_code: String,
}

pub mod admin;
pub mod url_management;
