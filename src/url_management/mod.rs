use url::Url;

pub mod api;
pub mod new_url;
pub mod route;

pub async fn is_absolute_url (url: &str) -> bool {
    Url::parse(url).is_ok()
}
