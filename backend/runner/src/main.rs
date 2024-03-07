use axum::Json;
use axum::routing::get;
use serde_json::{json, Value};
use socketioxide::SocketIo;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing_subscriber::FmtSubscriber;

mod ws;
mod fs;

#[tokio::main]
async fn main(){
    tracing::subscriber::set_global_default(FmtSubscriber::default()).unwrap();
    let (layer, io) = SocketIo::new_layer();
    io.ns("/", ws::on_connect);
    let app = axum::Router::new()
        .route("/", get(hello))
        .layer(ServiceBuilder::new()
            .layer(CorsLayer::permissive())
            .layer(layer));

    let listener = tokio::net::TcpListener::bind("localhost:3002").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> Json<Value> {
    Json(json!({"hello":23}))
}
