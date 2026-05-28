use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppCache {
    pub shuffle_state: HashMap<String, bool>,
    pub volume: u8
}

impl AppCache {
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let data = serde_json::to_string(self)?;
        
        fs::write(path, data)?;
        Ok(())
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let data = fs::read_to_string(path)?;
        
        Ok(serde_json::from_str(&data)?)
    }

    pub fn shuffle(&self, context_uri: &str) -> bool {
        self.shuffle_state.get(context_uri).copied().unwrap_or(false)
    }
}

impl Default for AppCache {
    fn default() -> Self {
        Self {
            shuffle_state: HashMap::new(),
            volume: 50
        }
    }
}
