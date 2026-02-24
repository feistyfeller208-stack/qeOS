use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::fs;
use std::path::PathBuf;
use dirs::home_dir;

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    pub id: String,
    pub created_at: i64,
}

impl Source {
    pub fn new() -> Self {
        Source {
            id: Uuid::new_v4().to_string(),
            created_at: chrono::Utc::now().timestamp(),
        }
    }
    
    fn get_path() -> PathBuf {
        let mut path = home_dir().unwrap();
        path.push(".qeos");
        fs::create_dir_all(&path).unwrap();
        path.push("source.json");
        path
    }
    
    pub fn load_or_create() -> Self {
        let path = Self::get_path();
        
        if path.exists() {
            let contents = fs::read_to_string(path).unwrap();
            serde_json::from_str(&contents).unwrap()
        } else {
            let source = Source::new();
            let json = serde_json::to_string_pretty(&source).unwrap();
            fs::write(path, json).unwrap();
            source
        }
    }
    
    pub fn reset() -> Self {
        let path = Self::get_path();
        let source = Source::new();
        let json = serde_json::to_string_pretty(&source).unwrap();
        fs::write(path, json).unwrap();
        source
    }
          }
