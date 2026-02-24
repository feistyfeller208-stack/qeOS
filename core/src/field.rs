use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::split::Split;
use crate::rule::{RuleEngine, Rule};

#[derive(Serialize, Deserialize, Debug)]
pub struct FieldState {
    pub splits: Vec<Split>,
    pub correlations: HashMap<String, f64>,
    pub last_updated: i64,
}

impl FieldState {
    pub fn new() -> Self {
        FieldState {
            splits: Vec::new(),
            correlations: HashMap::new(),
            last_updated: chrono::Utc::now().timestamp(),
        }
    }
    
    pub fn add_split(&mut self, split: Split) {
        self.splits.push(split);
        self.last_updated = chrono::Utc::now().timestamp();
        self.update_correlations();
    }
    
    pub fn update_correlations(&mut self) {
        // Keep only last 100 splits for performance
        if self.splits.len() > 100 {
            self.splits = self.splits[self.splits.len()-100..].to_vec();
        }
        
        // Group by context
        let mut context_map: HashMap<String, Vec<f64>> = HashMap::new();
        for split in &self.splits {
            context_map.entry(split.context.clone())
                .or_insert(Vec::new())
                .push(split.value);
        }
        
        // Calculate correlations between contexts
        let contexts: Vec<String> = context_map.keys().cloned().collect();
        for i in 0..contexts.len() {
            for j in i+1..contexts.len() {
                let ctx1 = &contexts[i];
                let ctx2 = &contexts[j];
                
                let vals1 = context_map.get(ctx1).unwrap();
                let vals2 = context_map.get(ctx2).unwrap();
                
                if vals1.len() > 1 && vals2.len() > 1 {
                    let corr = calculate_context_correlation(vals1, vals2);
                    self.correlations.insert(format!("{}:{}", ctx1, ctx2), corr);
                }
            }
        }
    }
    
    pub fn query_probability(&self, context: &str, rule_engine: &RuleEngine) -> f64 {
        // Find recent splits for this context
        let recent: Vec<&Split> = self.splits.iter()
            .filter(|s| s.context == context)
            .rev()
            .take(5)
            .collect();
        
        if recent.is_empty() {
            return rand::random::<f64>();
        }
        
        // Base probability from recent history
        let base: f64 = recent.iter().map(|s| s.value).sum::<f64>() / recent.len() as f64;
        
        // Apply rules
        let applicable_rules = rule_engine.get_applicable_rules(context);
        let mut final_value = base;
        
        for rule in applicable_rules {
            final_value = rule.apply(final_value);
        }
        
        final_value.max(0.0).min(1.0)
    }
}

fn calculate_context_correlation(vals1: &[f64], vals2: &[f64]) -> f64 {
    let min_len = vals1.len().min(vals2.len());
    if min_len < 2 {
        return 0.0;
    }
    
    let mean1 = vals1.iter().sum::<f64>() / vals1.len() as f64;
    let mean2 = vals2.iter().sum::<f64>() / vals2.len() as f64;
    
    let mut covariance = 0.0;
    let mut variance1 = 0.0;
    let mut variance2 = 0.0;
    
    for i in 0..min_len {
        let diff1 = vals1[i] - mean1;
        let diff2 = vals2[i] - mean2;
        covariance += diff1 * diff2;
        variance1 += diff1 * diff1;
        variance2 += diff2 * diff2;
    }
    
    if variance1 == 0.0 || variance2 == 0.0 {
        return 0.0;
    }
    
    covariance / (variance1.sqrt() * variance2.sqrt())
  }
