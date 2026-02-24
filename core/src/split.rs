use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Split {
    pub split_id: String,
    pub source_id: String,
    pub context: String,
    pub value: f64,
    pub timestamp: i64,
}

impl Split {
    pub fn new(source_id: &str, context: &str) -> Self {
        let mut rng = rand::thread_rng();
        
        Split {
            split_id: format!("{:x}", rng.gen::<u128>()),
            source_id: source_id.to_string(),
            context: context.to_string(),
            value: rng.gen::<f64>(),
            timestamp: Utc::now().timestamp(),
        }
    }
    
    pub fn from_source_with_seed(source_id: &str, context: &str, seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        
        Split {
            split_id: format!("{:x}", rng.gen::<u128>()),
            source_id: source_id.to_string(),
            context: context.to_string(),
            value: rng.gen::<f64>(),
            timestamp: Utc::now().timestamp(),
        }
    }
}

pub fn calculate_correlation(splits: &[Split]) -> f64 {
    if splits.len() < 2 {
        return 0.0;
    }
    
    let values: Vec<f64> = splits.iter().map(|s| s.value).collect();
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    
    let variance = values.iter()
        .map(|v| (v - mean).powi(2))
        .sum::<f64>() / values.len() as f64;
    
    if variance == 0.0 {
        return 0.0;
    }
    
    // Simple correlation with previous split
    let mut sum = 0.0;
    for i in 1..values.len() {
        sum += (values[i] - mean) * (values[i-1] - mean);
    }
    
    sum / (variance * (values.len() - 1) as f64)
  }
