use std::convert::Infallible;

use http_body_util::combinators::BoxBody;
use hyper::{
    body::{Bytes, Incoming},
    Method, Request, Response, StatusCode,
};
use tokio::sync::mpsc::Sender;

use crate::{
    controller::{
        todo::{create_todo_handle, delete_todo_handle, get_todo_handle, update_todo_handle},
        util::not_found,
    },
    db::Cmd,
    util::http_response::response_builder_fail,
};

pub async fn router(
    req: Request<Incoming>,
    tx: Sender<Cmd>,
) -> Response<BoxBody<Bytes, Infallible>> {
    let server_error_res =
        response_builder_fail(StatusCode::INTERNAL_SERVER_ERROR).expect("router error");

    match (req.method(), req.uri().path()) {
        // create todo from json
        //
        // [POST]
        // http://localhsot:3000/todo
        //
        // payload to create todo
        // json type
        // {
        //    "todo_text": "something to do"
        // }
        (&Method::POST, "/todo") => create_todo_handle(req, tx)
            .await
            .unwrap_or(server_error_res),
        // get all todo as a json
        // get as a array of json
        //
        // [GET]
        // http://localhsot:3000/todo
        (&Method::GET, "/todo") => get_todo_handle(req, tx).await.unwrap_or(server_error_res),
        // update todo selected by id
        //
        // [PUT]
        // http://localhsot:3000/todo
        //
        // payload to update todo
        // json type
        // {
        //    "target_id": "uuid",
        //    "todo_text": "update text",            <- Optional
        //    "todo_completed": "ture" or "false"    <- Optional
        // }
        (&Method::PUT, "/todo") => update_todo_handle(req, tx)
            .await
            .unwrap_or(server_error_res),
        // delete todo
        //
        // [DELETE]
        // http://localhsot:3000/todo
        //
        // payload to delete todo
        // json type
        // {
        //    "target_id": "uuid"
        // }
        (&Method::DELETE, "/todo") => delete_todo_handle(req, tx)
            .await
            .unwrap_or(server_error_res),
        // if uri is not match,
        // reaturn not found status code
        _ => not_found(req).await,
    }
}
