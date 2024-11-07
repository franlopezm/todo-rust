use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub text: String,
    pub done: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn create(text: String) -> Self {
        Task {
            id: Uuid::new_v4().to_string(),
            text,
            done: false,
            completed_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn completed(task: &mut Self, done: bool) {
        if done == task.done {
            return;
        }

        task.done = done;
        task.updated_at = Utc::now();

        if done {
            task.completed_at = Some(Utc::now());
        } else {
            task.completed_at = None;
        }
    }

    pub fn update_text(task: &mut Self, text: String) {
        task.text = text;
        task.updated_at = Utc::now();
    }
}
