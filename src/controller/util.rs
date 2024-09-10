use std::convert::Infallible;

use http_body_util::{combinators::BoxBody, Empty};
use hyper::{
    body::{Bytes, Incoming},
    Request, Response, StatusCode,
};

pub async fn not_found(_req: Request<Incoming>) -> Response<BoxBody<Bytes, Infallible>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(BoxBody::new(Empty::new()))
        .unwrap()
}
