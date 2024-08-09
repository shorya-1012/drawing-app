mod controllers;
mod models;
mod routes;

use models::draw_lines_models::{ClientState, DrawLine, RoomData};
use routes::setup_routes;
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing_subscriber::FmtSubscriber;

async fn on_connect(socket: SocketRef) {
    println!("socket connected : {}", socket.id);

    socket.on(
        "client-connect",
        |socket: SocketRef, Data::<RoomData>(data)| {
            match socket.join(data.room_code.clone()) {
                Ok(_) => println!("Join room successful"),
                Err(err) => println!("error joining room : \n {:#?}", err),
            };
            let _ = socket.within(data.room_code).emit("get-client-state", "");
        },
    );

    socket.on(
        "client-state",
        |socket: SocketRef, Data::<ClientState>(data)| {
            let _ = socket
                .within(data.room_code)
                .emit("server-state", data.state);
        },
    );

    socket.on(
        "clear-canvas",
        |socket: SocketRef, Data::<RoomData>(data)| {
            let _ = socket.within(data.room_code).emit("clear-canvas", "");
        },
    );

    socket.on("draw-line", |socket: SocketRef, Data::<DrawLine>(data)| {
        let _ = socket
            .within(data.room_code.clone())
            .emit("get-draw-line", data);
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Unable to setup TCP Listener");

    let (layer, io) = SocketIo::new_layer();

    io.ns("/socket", on_connect);

    let app = setup_routes().layer(
        ServiceBuilder::new()
            .layer(CorsLayer::permissive())
            .layer(layer),
    );

    match axum::serve(listener, app).await {
        Ok(_) => println!("Server started"),
        Err(err) => {
            println!("Unable to start server \n Reason :");
            println!("{:#?}", err)
        }
    }

    Ok(())
}
