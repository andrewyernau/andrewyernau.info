mod db;
mod routes;
mod models;

use actix_web::{App, HttpServer, web};
use actix_files::Files;
use db::create_db_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    let db_pool = create_db_pool().await;
    
    println!("Server running at http://127.0.0.1:8080");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .configure(routes::config_routes)
            .service(Files::new("/static", "./static").prefer_utf8(true))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}