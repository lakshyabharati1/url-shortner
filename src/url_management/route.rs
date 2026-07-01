use std::error::Error;

use crate::AppState;

pub async fn router(route: String, state: AppState) -> Result<Option<String>, Box<dyn Error>> {
    let tree = state.url_db;

    let url_bytes = tree.get(route)?;

    match url_bytes {
        Some(url_bytes) => {
            let url = String::from_utf8(url_bytes.to_vec())?;
            return Ok(Some(url));
        }
        None => {
            return Ok(None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::router;
    use crate::AppState;

    #[tokio::test]
    async fn router_returns_stored_url() {
        let db = sled::Config::new().temporary(true).open().unwrap();
        let tree = db.open_tree("url").unwrap();
        tree.insert("abc", "https://example.com").unwrap();

        let state = AppState {
            url_db: tree,
            admin_verification_code: String::from("1234"),
        };

        let result = router(String::from("abc"), state).await.unwrap();

        assert_eq!(result, Some(String::from("https://example.com")));
    }

    #[tokio::test]
    async fn router_returns_none_for_missing_key() {
        let db = sled::Config::new().temporary(true).open().unwrap();
        let tree = db.open_tree("url").unwrap();

        let state = AppState {
            url_db: tree,
            admin_verification_code: String::from("1234"),
        };

        let result = router(String::from("missing"), state).await.unwrap();

        assert_eq!(result, None);
    }
}
