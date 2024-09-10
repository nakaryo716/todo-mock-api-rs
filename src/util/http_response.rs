use std::convert::Infallible;

use http_body_util::{combinators::BoxBody, Empty};
use hyper::{body::Bytes, Response, StatusCode};

pub fn response_builder_as_json(
    res_body: String,
) -> Result<Response<BoxBody<Bytes, Infallible>>, Box<dyn std::error::Error>> {
    let res = Response::builder()
        .header("Content-Type", mime::APPLICATION_JSON.to_string())
        .status(StatusCode::OK)
        .body(BoxBody::new(res_body))?;
    Ok(res)
}

pub fn response_builder_fail(
    status: StatusCode,
) -> Result<Response<BoxBody<Bytes, Infallible>>, Box<dyn std::error::Error>> {
    let res = Response::builder()
        .status(status)
        .body(BoxBody::new(Empty::new()))?;
    Ok(res)
}
