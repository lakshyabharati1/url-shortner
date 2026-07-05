use crate::AppState;
use std::error::Error;

pub async fn new_url(url: &str, state: AppState) -> Result<String, Box<dyn Error>> {
    let tree = state.url_db;

    let mut last = state.last_url.lock().await;
    let bytes = last.as_bytes();
    let key = increment(&bytes).await;
    tree.insert(key.as_slice(), url.as_bytes())?;

    *last = String::from_utf8(key.to_vec())?;

    Ok(String::from_utf8(key)?)
}

pub async fn increment(vec: &[u8]) -> Vec<u8> {
    let mut incremented_char = vec.to_vec();
    incremented_char.reverse();

    let mut carry: u8 = 1;

    for i in incremented_char.iter_mut() {
        if carry == 0 {
            break;
        }

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

pub async fn decrement(vec: &[u8]) -> Vec<u8> {
    let mut decremented_char = vec.to_vec();
    decremented_char.reverse();

    let mut borrow: u8 = 1;

    for i in decremented_char.iter_mut() {
        if borrow == 0 {
            break;
        }

        if *i == b'a' {
            borrow = 1;
            *i = b'z';
        } else {
            *i = *i - borrow;
            borrow = 0;
        }
    }

    decremented_char.reverse();

    if borrow == 1 {
        decremented_char.pop();
    }

    decremented_char
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use tokio::sync::Mutex;

    use super::{decrement, increment, new_url};
    use crate::AppState;

    #[tokio::test]
    async fn increment_rolls_over_z_to_aa() {
        assert_eq!(increment(b"z").await, b"aa".to_vec());
        assert_eq!(increment(b"az").await, b"ba".to_vec());
        assert_eq!(increment(b"bz").await, b"ca".to_vec());
        assert_eq!(increment(b"za").await, b"zb".to_vec());
        assert_eq!(increment(b"zz").await, b"aaa".to_vec());
    }

    #[tokio::test]
    async fn decrement_rolls_over_aa_to_z() {
        assert_eq!(decrement(b"aa").await, b"z".to_vec());
        assert_eq!(decrement(b"ba").await, b"az".to_vec());
        assert_eq!(decrement(b"ca").await, b"bz".to_vec());
        assert_eq!(decrement(b"zb").await, b"za".to_vec());
        assert_eq!(decrement(b"aaa").await, b"zz".to_vec());
    }

    #[tokio::test]
    async fn new_url_inserts_and_returns_key() {
        let db = sled::Config::new().temporary(true).open().unwrap();
        let tree = db.open_tree("url").unwrap();

        let state = AppState {
            url_db: tree.clone(),
            admin_verification_code: String::from("1234"),
            last_url: Arc::new(Mutex::new(String::from("a"))),
        };

        let key = new_url("https://example.com", state).await.unwrap();

        assert_eq!(key, "b");
        assert_eq!(tree.get("b").unwrap().unwrap(), b"https://example.com".to_vec());
    }
}
