use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteTodo {
    target_id: String,
}

impl DeleteTodo {
    pub fn get_target_id(&self) -> &str {
        &self.target_id
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Todo {
    id: String,
    todo_text: String,
    completed: bool,
}

impl Todo {
    pub fn new(payload: CreateTodo) -> Self {
        Todo {
            id: Uuid::new_v4().to_string(),
            todo_text: payload.todo_text,
            completed: false,
        }
    }

    pub fn get_id(&self) -> &str {
        self.id.as_ref()
    }

    pub fn modify_text(&mut self, text: &str) {
        self.todo_text = text.to_string();
    }

    pub fn modify_completed(&mut self, completed: bool) {
        self.completed = completed;
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTodo {
    todo_text: String,
}

impl CreateTodo {
    pub fn new(todo_text: &str) -> Self {
        CreateTodo {
            todo_text: todo_text.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTodo {
    target_id: String,
    todo_text: Option<String>,
    completed: Option<bool>,
}

impl UpdateTodo {
    pub fn new(target_id: String) -> Self {
        Self {
            target_id,
            todo_text: None,
            completed: None,
        }
    }

    pub fn have_text(&self) -> bool {
        self.todo_text.is_some()
    }

    pub fn have_completed(&self) -> bool {
        self.completed.is_some()
    }

    pub fn get_target_id(&self) -> &str {
        &self.target_id
    }

    pub fn get_text(&self) -> &Option<String> {
        &self.todo_text
    }

    pub fn get_completed(&self) -> &Option<bool> {
        &self.completed
    }

    pub fn set_text(&mut self, payload_text: &str) -> &UpdateTodo {
        self.todo_text = Some(payload_text.to_string());

        let update_todo_asref: &UpdateTodo = self;
        update_todo_asref
    }

    pub fn set_completed(&mut self, payload_completed: bool) -> &UpdateTodo {
        self.completed = Some(payload_completed);

        let update_todo_asref: &UpdateTodo = self;
        update_todo_asref
    }
}
