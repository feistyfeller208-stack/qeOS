use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use dirs::home_dir;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RuleType {
    Direct,
    Inverse,
    Delayed,
    Cascade,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rule {
    pub id: String,
    pub name: String,
    pub rule_type: RuleType,
    pub contexts: Vec<String>,
    pub strength: f64,
    pub condition: Option<String>,
    pub decay: Option<String>,
    pub active: bool,
    pub times_applied: u32,
}

impl Rule {
    pub fn new(name: &str, rule_type: RuleType, contexts: Vec<String>, strength: f64) -> Self {
        Rule {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            rule_type,
            contexts,
            strength,
            condition: None,
            decay: None,
            active: true,
            times_applied: 0,
        }
    }
    
    pub fn apply(&mut self, value: f64) -> f64 {
        self.times_applied += 1;
        
        match self.rule_type {
            RuleType::Direct => value * (1.0 + self.strength),
            RuleType::Inverse => 1.0 - (value * (1.0 - self.strength)),
            RuleType::Delayed => value, // Simplified
            RuleType::Cascade => value, // Simplified
        }
    }
}

pub struct RuleEngine {
    rules: Vec<Rule>,
}

impl RuleEngine {
    pub fn new() -> Self {
        RuleEngine {
            rules: Vec::new(),
        }
    }
    
    pub fn load_all() -> Self {
        let mut engine = RuleEngine::new();
        let mut path = home_dir().unwrap();
        path.push(".qeos/rules");
        
        if path.exists() {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    if let Ok(contents) = fs::read_to_string(entry.path()) {
                        if let Ok(rule) = serde_json::from_str::<Rule>(&contents) {
                            engine.rules.push(rule);
                        }
                    }
                }
            }
        }
        
        engine
    }
    
    pub fn add_rule(&mut self, rule: Rule) {
        let mut path = home_dir().unwrap();
        path.push(format!(".qeos/rules/{}.json", rule.id));
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let json = serde_json::to_string_pretty(&rule).unwrap();
        fs::write(path, json).unwrap();
        self.rules.push(rule);
    }
    
    pub fn get_applicable_rules(&self, context: &str) -> Vec<&Rule> {
        self.rules.iter()
            .filter(|r| r.active && r.contexts.iter().any(|c| c == context))
            .collect()
    }
}
