use std::collections::HashMap; 
use std::error::Error; 
use tokio::fs::File; 
use tokio::io::AsyncReadExt; 
use serde_json::Value;

pub async fn get_json_value(file_path: &str) -> Result<HashMap<String, Value>, Box<dyn Error>> {
    let mut file = File::open(file_path).await?; 
    let mut contents = String::new(); 
    file.read_to_string(&mut contents).await?; 
    let data: HashMap<String, Value> = serde_json::from_str(&contents)?; 
    Ok(data) 
}