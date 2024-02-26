use std::path::Path;
use serde_json::json;
use socketioxide::extract::{Data, SocketRef, State};
use socketioxide::layer::SocketIoLayer;
use socketioxide::SocketIo;
use tracing::{info, warn};
use crate::fs::fetch_dir;


pub async fn on_connect(socket: SocketRef){
    info!("Socket connected {}", socket.id);
    let host = socket.req_parts().headers.get("host");
    let host_name = String::from("DeeppinkFrigidAudacity.bala.com");
    if let Some(host) = host{
        let host_name = host.to_str().unwrap();
    }
    let repl_id = host_name.split(".").collect::<Vec<&str>>()[0];

    init_handlers(socket, repl_id.to_string());
    // socket.on("connection", |socket: SocketRef| {
    //
    //     let host = socket.req_parts().headers.get("host");
    //     let host_name = String::from("RandomString.bala.com");
    //     if let Some(host) = host{
    //         let host_name = host.to_str().unwrap();
    //     }
    //     let repl_id = host_name.split(".").collect::<Vec<&str>>()[0];
    //
    //     // socket.emit("loaded", json!({
    //     //     "rootContent": fer
    //     // }))
    //     init_handlers(socket, repl_id.to_string());
    // });
}

fn init_handlers(socket: SocketRef, repl_id: String){
    socket.on("fetchDir", |socket: SocketRef, Data::<String>(dir)| async move{
        info!("I am in fetchDir {}", repl_id);
        let current_dir = Path::new("/Users/balasubramanyam");
        let full_path = current_dir.join("tmp").join(&repl_id).join(&dir);
        let data = fetch_dir(full_path, &dir);
        let _ = socket.emit("resolveFetchDir", json!({
            "result": data
        }));
    });
    socket.on("join", |socket: SocketRef, Data::<String>(room_id) | async move {
        warn!("Joining room {}", room_id);
        let _ = socket.leave_all();
        let _ = socket.join(room_id);
        let _ = socket.emit("messages",{});
        let _ = socket.emit("connection", {});
    });

    socket.on("message",  |socket: SocketRef, Data::<String>(data)| async move{
        info!("message received {:?}", data);
        let _ = socket.within(data).emit("message", {});
    });
}