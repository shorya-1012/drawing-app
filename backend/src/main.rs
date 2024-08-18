mod controllers;
mod models;
mod routes;
mod utils;

use std::{process::exit, sync::Arc};

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
use sqlx::{Pool, Postgres};
use tokio::{net::TcpListener, sync::Mutex};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing_subscriber::FmtSubscriber;

async fn on_connect(socket: SocketRef, db: Arc<Pool<Postgres>>) {
    println!("socket connected : {}", socket.id);

    let db_clone = Arc::clone(&db);
    socket.on_disconnect(move |s: SocketRef, reason: DisconnectReason| {
        println!("{:#?}", s.rooms());
        tokio::spawn(async move {
            let socket_id = s.id.to_string();

            let query = "DELETE FROM rooms_users WHERE socket_id = $1";

            let result = sqlx::query(query)
                .bind(&socket_id)
                .execute(&*db_clone)
                .await;

            match result {
                Ok(_) => println!("Deleted row from rooms_users table successfully"),
                Err(err) => println!("Error deleting row from rooms_users table: {:?}", err),
            };
        });
    });

    let db_clone = Arc::clone(&db);
    socket.on(
        "client-connect",
        move |socket: SocketRef, Data::<RoomData>(data)| {
            println!("client connect");
            tokio::spawn(async move {
                match socket.join(data.room_code.clone()) {
                    Ok(_) => println!("Connect room successful"),
                    Err(err) => println!("error joining room : \n {:#?}", err),
                };

                let query =
                    "INSERT INTO rooms_users (room_id, user_id, socket_id) VALUES ($1, $2, $3)";

                let user_id = &data.user_id;
                let socket_id = socket.id.to_string();

                let insert_result = sqlx::query(query)
                    .bind(&data.room_code)
                    .bind(user_id)
                    .bind(socket_id)
                    .execute(&*db_clone)
                    .await;

                match insert_result {
                    Ok(_) => println!("Insert into rooms_users successful"),
                    Err(err) => println!("Error inserting into rooms_users: {:?}", err),
                };

                let _ = socket.within(data.room_code).emit("get-client-state", "");
            });
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

    let db_pool_clone = Arc::new(db_pool.clone());

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Unable to setup TCP Listener");
    let (layer, io) = SocketIo::new_layer();

    let app_state = Arc::new(Mutex::new(InternalState {
        db: db_pool,
        redis: redis_client,
    }));

    // io.ns("/socket", |socket: SocketRef| async {
    //     let db = Arc::clone(&d);
    //     on_connect(socket, db).await;
    // });

    io.ns("/socket", {
        let db = Arc::clone(&db_pool_clone);
        move |socket: SocketRef| {
            let d = Arc::clone(&db);
            async move {
                on_connect(socket, d).await;
            }
        }
    });

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
