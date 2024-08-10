mod controllers;
mod models;
mod routes;

use std::{process::exit, sync::Arc};

use dotenv::dotenv;
use models::draw_lines_models::{ClientState, DrawLine, RoomData};
use routes::setup_routes;
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use tokio::{net::TcpListener, sync::Mutex};
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

    dotenv().ok();

    let db_url = std::env::var("DB_URL").expect("DB URL NOT FOUND");

    let db_pool = Arc::new(Mutex::new(
        match sqlx::postgres::PgPool::connect(&db_url).await {
            Ok(pool) => {
                println!("Successfully connected to Db");
                pool
            }
            Err(err) => {
                println!("{:#?}", err);
                exit(1)
            }
        },
    ));

    // let pool = db_pool.clone().lock().await;

    // runs migrations
    // match sqlx::migrate!("./migrations").run(&*pool).await {
    //     Ok(_) => println!("Migrations made successfully"),
    //     Err(err) => {
    //         println!("Error while making migrations \n {:#?}", err);
    //         std::process::exit(1)
    //     }
    // }

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Unable to setup TCP Listener");

    let (layer, io) = SocketIo::new_layer();
    io.ns("/socket", on_connect);

    let app = setup_routes(db_pool).layer(
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
