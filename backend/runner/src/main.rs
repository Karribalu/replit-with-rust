mod ws;
mod state;

use actix_web::{App, HttpServer, web};
use socketioxide::extract::{Data, SocketRef, State};
use socketioxide::SocketIo;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, serde::Deserialize)]
struct MessageIn {
    room: String,
    text: String,
}

#[derive(serde::Serialize)]
struct Messages {
    messages: Vec<state::Message>,
}

async fn on_connect(socket: SocketRef){
    info!("socket connected {}", socket.id);

    socket.on(
        "join",
        |socket: SocketRef, Data::<String>(room), store: State<state::MessageStore>| async move {
            info!("Received join: {:?}", room);
            let _ = socket.leave_all();
            let _ = socket.join(room.clone());
            let messages = store.get(&room).await;
            let _ = socket.emit("messages", Messages{messages});
        }
    )
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    let messages = state::MessageStore::default();
    let (layer, io) = SocketIo::builder().with_state(messages).build_layer();
    // io.ns("/", )
    HttpServer::new(|| App::new()
        .route("/", web::get().to("hello"))
        .route("/project", web::get().to(services::project_service::create_project)))
        .workers(4)
        .bind(("localhost",8080))?
        .run()
        .await
}