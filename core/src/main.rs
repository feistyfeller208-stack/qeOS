mod source;
mod split;
mod rule;
mod field;
mod api;

use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use source::Source;
use field::FieldState;

struct AppState {
    source: Mutex<Source>,
    field: Mutex<FieldState>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 qeOS Core starting...");
    
    let source = Source::load_or_create();
    println!("📇 Source ID: {}", source.id);
    
    let field = FieldState::new();
    
    let app_state = web::Data::new(AppState {
        source: Mutex::new(source),
        field: Mutex::new(field),
    });
    
    println!("🌐 Listening on http://localhost:3030");
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(api::health)
            .service(api::get_source)
            .service(api::create_split)
            .service(api::query)
            .service(api::register_rule)
    })
    .bind(("127.0.0.1", 3030))?
    .run()
    .await
  }
