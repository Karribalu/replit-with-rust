use actix_web::{App, HttpServer, web};

mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    HttpServer::new(|| App::new()
        .route("/", web::get().to(services::hello::search))
        .route("/project", web::get().to(services::project_service::create_project)))
        .workers(4)
        .bind(("localhost",8080))?
        .run()
        .await
}
