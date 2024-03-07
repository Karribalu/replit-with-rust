use axum::http::{HeaderValue, Method};
use axum::http::header::CONTENT_TYPE;
use axum::Router;
use axum::routing::{get, post};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::FmtSubscriber;

mod services;

#[tokio::main]
async fn main(){
    dotenv::dotenv().ok();
    tracing::subscriber::set_global_default(FmtSubscriber::default()).unwrap();
    let app = Router::new()
        .layer(CorsLayer::new()
                   .allow_origin(Any)
                   .allow_headers([CONTENT_TYPE])
                   .allow_methods([Method::GET, Method::POST]),)
        .route("/", get(services::hello::search))
        .route("/project", post(services::project_service::create_project));
    let listener = tokio::net::TcpListener::bind("localhost:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}
