use std::{error::Error, sync::Arc};
use sled::Tree;
use tokio::sync::Mutex;

use crate::url_management::new_url::{decrement, increment};

#[derive(Clone)]
pub struct AppState {
    pub url_db: sled::Tree,
    pub admin_verification_code: String,
    pub last_url: Arc<Mutex<String>>,
}

pub mod admin;
pub mod url_management;

pub async fn get_last_url(url_tree: Tree) -> Result<String, Box<dyn Error>> {
    let mut temp = match url_tree.last()? {
            Some(x) => x.0.to_vec(), 
            None => b"".to_vec(),
        };

    println!("{}", String::from_utf8(temp.to_vec())?);

    if !temp.ends_with(b"z") {
        return Ok(String::from_utf8(temp)?)
    }
    

    println!("{}", url_tree.contains_key(&increment(&temp).await)?);
    
    
    if !url_tree.contains_key(&increment(&temp).await)? {
        return Ok(String::from_utf8(temp)?)
    }
    
    let mut condition = true;

    println!("{:#?}", temp);
    
    while condition {
        temp = increment(&temp).await;
        
        condition = url_tree.contains_key(&temp)?;
        println!("{}", condition)
    }
    println!("{:#?}", temp);

    temp = decrement(&temp).await;
    println!("{:#?}", temp);
    return Ok(String::from_utf8(temp.to_vec())?);
}
