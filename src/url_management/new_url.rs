use axum::handler::HandlerWithoutStateExt;

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
