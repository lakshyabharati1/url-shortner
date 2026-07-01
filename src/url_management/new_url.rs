use crate::AppState;
use std::error::Error;

pub async fn new_url(url: &str, state: AppState) -> Result<String, Box<dyn Error>> {
    let tree = state.url_db;

    let key = match tree.last()? {
        Some((key, _val)) => increment(&key).await,
        None => vec![b'a'],
    };

    tree.insert(key.as_slice(), url.as_bytes())?;

    Ok(String::from_utf8(key)?)
}

pub async fn increment(vec: &[u8]) -> Vec<u8> {
    let mut incremented_char = vec.to_vec();
    incremented_char.reverse();

    let mut carry: u8 = 1;

    for i in incremented_char.iter_mut() {
        if *i == b'z' {
            carry = 1;
            *i = b'a';
        } else {
            *i = *i + carry;
            carry = 0;
        }
    }

    incremented_char.reverse();

    if carry == 1 {
        incremented_char.push(b'a');
    }

    incremented_char
}

#[cfg(test)]
mod tests {
    use super::{increment, new_url};
    use crate::AppState;

    #[tokio::test]
    async fn increment_rolls_over_z_to_aa() {
        assert_eq!(increment(b"z").await, b"aa".to_vec());
        assert_eq!(increment(b"az").await, b"ba".to_vec());
        assert_eq!(increment(b"bz").await, b"ca".to_vec());
    }

    #[tokio::test]
    async fn new_url_inserts_and_returns_key() {
        let db = sled::Config::new().temporary(true).open().unwrap();
        let tree = db.open_tree("url").unwrap();

        let state = AppState {
            url_db: tree.clone(),
            admin_verification_code: String::from("1234"),
        };

        let key = new_url("https://example.com", state).await.unwrap();

        assert_eq!(key, "a");
        assert_eq!(tree.get("a").unwrap().unwrap(), b"https://example.com".to_vec());
    }
}
