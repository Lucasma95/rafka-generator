use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Request {
    pub message: String,
    pub queue_name: String,
    pub producer: String,
}