use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskBody {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskDone {
    pub done: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskError {
    pub error: String,
    pub message: String,
}
