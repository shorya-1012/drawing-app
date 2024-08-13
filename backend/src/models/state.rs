use std::sync::Arc;

use redis::Client;
use sqlx::{Pool, Postgres};
use tokio::sync::Mutex;

pub struct InternalState {
    pub db: Pool<Postgres>,
    pub redis: Client,
}

pub type AppState = Arc<Mutex<InternalState>>;
// pub struct AppState {
//     pub db :
// }
