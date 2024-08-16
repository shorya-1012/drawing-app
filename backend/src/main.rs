mod controllers;
mod models;
mod routes;
mod utils;

use std::{process::exit, sync::Arc};

// use http::header::

use axum::http::Method;
use dotenv::dotenv;
use models::{
    draw_lines_models::{ClientState, DrawLine, RoomData},
    state::InternalState,
};
use routes::setup_routes;
use socketioxide::{
    extract::{Data, SocketRef},
    socket::DisconnectReason,
    SocketIo,
};
use tokio::{net::TcpListener, sync::Mutex};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::FmtSubscriber;

async fn on_connect(socket: SocketRef) {
    println!("socket connected : {}", socket.id);

    socket.on_disconnect(|s: SocketRef, reason: DisconnectReason| {
        // println!("Socket {} was disconnected because {} ", s.id, reason);
        println!("{:#?}", s.rooms());
    });

    socket.on(
        "client-connect",
        |socket: SocketRef, Data::<RoomData>(data)| {
            match socket.join(data.room_code.clone()) {
                Ok(_) => println!("Connect room successful"),
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

    let db_pool = match sqlx::postgres::PgPool::connect(&db_url).await {
        Ok(pool) => {
            println!("Successfully connected to Db");
            pool
        }
        Err(err) => {
            println!("{:#?}", err);
            exit(1)
        }
    };

    let redis_client = redis::Client::open("redis://127.0.0.1/")?;

    // runs migrations
    match sqlx::migrate!("./migrations").run(&db_pool.clone()).await {
        Ok(_) => println!("Migrations made successfully"),
        Err(err) => {
            println!("Error while making migrations \n {:#?}", err);
            std::process::exit(1)
        }
    }

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Unable to setup TCP Listener");
    let (layer, io) = SocketIo::new_layer();
    io.ns("/socket", on_connect);

    let app_state = Arc::new(Mutex::new(InternalState {
        db: db_pool,
        redis: redis_client,
    }));

    let origin = ["http://localhost:5173".parse()?];

    let app = setup_routes(app_state).layer(
        ServiceBuilder::new()
            .layer(
                CorsLayer::new()
                    .allow_origin(origin)
                    .allow_methods([Method::GET, Method::POST])
                    .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
                    .allow_credentials(true),
            )
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
