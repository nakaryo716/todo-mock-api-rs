use std::convert::Infallible;

use http_body_util::{combinators::BoxBody, Empty};
use hyper::{
    body::{Bytes, Incoming},
    Request, Response, StatusCode,
};

use tokio::sync::{mpsc::Sender, oneshot};

use crate::{
    db::{Cmd, ResponseMessage},
    model::todo::{DeleteTodo, UpdateTodo},
    util::{
        body::deserialize_body,
        http_response::{response_builder_as_json, response_builder_fail},
    },
};

pub async fn create_todo_handle(
    req: Request<Incoming>,
    tx: Sender<Cmd>,
) -> Result<Response<BoxBody<Bytes, Infallible>>, Box<dyn std::error::Error>> {
    let (sender, reciver) = oneshot::channel();

    let deserialized_payload = deserialize_body(req).await;

    match deserialized_payload {
        Ok(payload) => {
            let cmd = Cmd::Create(payload, sender);
            tx.send(cmd).await?;

            match reciver.await? {
                ResponseMessage::Ok(todo) => {
                    let res_body = serde_json::to_string(&todo)?;
                    response_builder_as_json(res_body)
                }
                _ => response_builder_fail(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Err(_e) => response_builder_fail(StatusCode::BAD_REQUEST),
    }
}

pub async fn get_todo_handle(
    _req: Request<Incoming>,
    tx: Sender<Cmd>,
) -> Result<Response<BoxBody<Bytes, Infallible>>, Box<dyn std::error::Error>> {
    let (sender, reciver) = oneshot::channel();

    let cmd = Cmd::Read(sender);
    tx.send(cmd).await?;

    match reciver.await? {
        ResponseMessage::Ok(todos) => {
            let todos = serde_json::to_string(&todos)?;
            response_builder_as_json(todos)
        }
        _ => response_builder_fail(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_todo_handle(
    req: Request<Incoming>,
    tx: Sender<Cmd>,
) -> Result<Response<BoxBody<Bytes, Infallible>>, Box<dyn std::error::Error>> {
    let (sender, receiver) = oneshot::channel();

    let todo_payload;

    if let Ok(update) = deserialize_body::<UpdateTodo>(req).await {
        todo_payload = update;
    } else {
        return response_builder_fail(StatusCode::BAD_REQUEST);
    }

    let cmd = Cmd::Update(
        todo_payload.get_target_id().to_string(),
        todo_payload,
        sender,
    );
    tx.send(cmd).await?;

    match receiver.await? {
        ResponseMessage::Ok(modified_todo) => {
            let res_body = serde_json::to_string(&modified_todo)?;
            response_builder_as_json(res_body)
        }
        ResponseMessage::NotFound => response_builder_fail(StatusCode::NOT_FOUND),
        _ => response_builder_fail(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_todo_handle(
    req: Request<Incoming>,
    tx: Sender<Cmd>,
) -> Result<Response<BoxBody<Bytes, Infallible>>, Box<dyn std::error::Error>> {
    let (sender, receiver) = oneshot::channel();

    let todo_payload;
    if let Ok(update) = deserialize_body::<DeleteTodo>(req).await {
        todo_payload = update;
    } else {
        return response_builder_fail(StatusCode::BAD_REQUEST);
    }
    let id = todo_payload.get_target_id();

    let cmd = Cmd::Delete(id.to_string(), sender);
    tx.send(cmd).await?;

    match receiver.await? {
        ResponseMessage::NoContents => {
            let res = Response::builder()
                .status(StatusCode::NO_CONTENT)
                .body(BoxBody::new(Empty::new()))?;
            Ok(res)
        }
        ResponseMessage::NotFound => response_builder_fail(StatusCode::NOT_FOUND),
        _ => response_builder_fail(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
