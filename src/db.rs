use crate::tasks::models::Task;
use std::sync::Mutex;

pub struct AppState {
    pub task: Mutex<Vec<Task>>,
}
