mod controllers;
mod models;
mod routes;

use models::draw_lines_models::DrawLine;
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
        |socket: SocketRef, Data::<Option<String>>(data)| {
            let _ = socket.broadcast().emit("get-client-state", data);
        },
    );

    socket.on("client-state", |socket: SocketRef, Data::<String>(data)| {
        let _ = socket.broadcast().emit("server-state", data);
    });

    socket.on(
        "clear-canvas",
        |socket: SocketRef, Data::<Option<String>>(data)| {
            let _ = socket.broadcast().emit("clear-canvas", data);
        },
    );

    socket.on("draw-line", |socket: SocketRef, Data::<DrawLine>(data)| {
        let _ = socket.broadcast().emit("get-draw-line", data);
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Unable to setup TCP Listener");

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);

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
