use deadpool_lapin::Pool;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AppState {
    // pub db_pool: DatabaseConnection
    pub amqp_pool: Pool,
}

#[derive(Deserialize, Serialize)]
pub struct MessageData {
    pub run_code: String,
    pub process_type: String,
    pub key_message_name: String,
    pub status: String,
    pub status_message: String,
}
