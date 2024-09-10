use std::{collections::HashMap, future::Future, pin::Pin};

use tokio::sync::{mpsc::Receiver, oneshot};

use crate::model::todo::{CreateTodo, Todo, UpdateTodo};

#[derive(Debug, Clone)]
pub struct TodoDb {
    pub todo_pool: HashMap<String, Todo>,
}

impl TodoDb {
    fn new() -> Self {
        TodoDb {
            todo_pool: HashMap::default(),
        }
    }

    fn insert_todo(&mut self, new_todo: CreateTodo) -> Todo {
        let todo = Todo::new(new_todo);
        let id = todo.get_id();
        self.todo_pool.insert(id.to_owned(), todo.clone());
        todo
    }

    fn get_all_todo(&self) -> Vec<Todo> {
        self.todo_pool
            .iter()
            .map(|(_id, todo)| todo.to_owned())
            .collect()
    }

    fn update_todo(&mut self, id: &str, update_todo: UpdateTodo) -> Option<Todo> {
        if let Some(todo) = self.todo_pool.get_mut(id) {
            if update_todo.have_text() {
                // it is safe to use .unwrap()
                let txt = update_todo.get_text().to_owned().unwrap();
                todo.modify_text(txt.as_str());
            }

            if update_todo.have_completed() {
                // it is safe to use .unwrap()
                let completed = update_todo.get_completed().to_owned().unwrap();
                todo.modify_completed(completed);
            }

            let todo = self.todo_pool.get(id).unwrap().to_owned();
            Some(todo)
        } else {
            None
        }
    }

    fn delete_todo(&mut self, id: &str) -> Option<Todo> {
        self.todo_pool.remove(id)
    }
}

#[derive(Debug)]
pub enum Cmd {
    Create(CreateTodo, oneshot::Sender<ResponseMessage<Todo>>),
    Read(oneshot::Sender<ResponseMessage<Vec<Todo>>>),
    Update(String, UpdateTodo, oneshot::Sender<ResponseMessage<Todo>>),
    Delete(String, oneshot::Sender<ResponseMessage<Todo>>),
}

#[derive(Debug)]
pub enum ResponseMessage<B> {
    Ok(B),
    NoContents,
    NotFound,
}

// database task
pub fn db_task(mut rx: Receiver<Cmd>) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
    Box::pin(async move {
        // initialize database
        let mut db = TodoDb::new();

        while let Some(msg) = rx.recv().await {
            match msg {
                Cmd::Create(payload, sender) => {
                    let todo = db.insert_todo(payload);
                    send_message_to_handler(sender, ResponseMessage::Ok(todo))
                }
                Cmd::Read(sender) => {
                    let todos = db.get_all_todo();
                    send_message_to_handler(sender, ResponseMessage::Ok(todos))
                }
                Cmd::Update(id, update_todo, sender) => {
                    if let Some(todo) = db.update_todo(&id, update_todo) {
                        send_message_to_handler(sender, ResponseMessage::Ok(todo))
                    } else {
                        send_message_to_handler(sender, ResponseMessage::NotFound)
                    }
                }
                Cmd::Delete(id, sender) => {
                    if db.delete_todo(&id).is_some() {
                        send_message_to_handler(sender, ResponseMessage::NoContents)
                    } else {
                        send_message_to_handler(sender, ResponseMessage::NoContents)
                    }
                }
            }
        }
    })
}

fn send_message_to_handler<T>(
    sender: oneshot::Sender<ResponseMessage<T>>,
    message: ResponseMessage<T>,
) {
    if sender.send(message).is_err() {
        println!("failed to send message");
    }
}
