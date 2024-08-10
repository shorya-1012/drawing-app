use std::sync::Arc;

use sqlx::{Pool, Postgres};
use tokio::sync::Mutex;

pub type AppState = Arc<Mutex<Pool<Postgres>>>;
// pub struct AppState {
//     pub db :
// }
