use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::split::Split;
use crate::rule::{Rule, RuleEngine, RuleType};

#[derive(Deserialize)]
pub struct SplitRequest {
    context: String,
}

#[derive(Serialize)]
pub struct SplitResponse {
    split_id: String,
    value: f64,
}

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "version": "0.1.0"
    }))
}

#[get("/source")]
pub async fn get_source(data: web::Data<AppState>) -> impl Responder {
    let source = data.source.lock().unwrap();
    HttpResponse::Ok().json(serde_json::json!({
        "id": source.id,
        "created_at": source.created_at
    }))
}

#[post("/split")]
pub async fn create_split(
    req: web::Json<SplitRequest>,
    data: web::Data<AppState>
) -> impl Responder {
    let source = data.source.lock().unwrap();
    let split = Split::new(&source.id, &req.context);
    
    let mut field = data.field.lock().unwrap();
    field.add_split(split.clone());
    
    HttpResponse::Ok().json(SplitResponse {
        split_id: split.split_id,
        value: split.value,
    })
}

#[derive(Deserialize)]
pub struct QueryRequest {
    context: String,
}

#[derive(Serialize)]
pub struct QueryResponse {
    probability: f64,
    confidence: f64,
    correlated_contexts: Vec<String>,
}

#[post("/query")]
pub async fn query(
    req: web::Json<QueryRequest>,
    data: web::Data<AppState>
) -> impl Responder {
    let field = data.field.lock().unwrap();
    let rule_engine = RuleEngine::load_all();
    
    let probability = field.query_probability(&req.context, &rule_engine);
    
    // Find correlated contexts
    let mut correlated = Vec::new();
    for (key, value) in &field.correlations {
        if key.contains(&req.context) && value.abs() > 0.5 {
            let parts: Vec<&str> = key.split(':').collect();
            for part in parts {
                if part != req.context {
                    correlated.push(part.to_string());
                }
            }
        }
    }
    
    HttpResponse::Ok().json(QueryResponse {
        probability,
        confidence: 0.7, // Simplified
        correlated_contexts: correlated,
    })
}

#[derive(Deserialize)]
pub struct RuleRequest {
    name: String,
    rule_type: String,
    contexts: Vec<String>,
    strength: f64,
}

#[post("/rules")]
pub async fn register_rule(
    req: web::Json<RuleRequest>,
    data: web::Data<AppState>
) -> impl Responder {
    let rule_type = match req.rule_type.as_str() {
        "direct" => RuleType::Direct,
        "inverse" => RuleType::Inverse,
        "delayed" => RuleType::Delayed,
        "cascade" => RuleType::Cascade,
        _ => RuleType::Direct,
    };
    
    let rule = Rule::new(&req.name, rule_type, req.contexts.clone(), req.strength);
    
    let mut rule_engine = RuleEngine::load_all();
    rule_engine.add_rule(rule);
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": "created",
        "name": req.name
    }))
                                              }
