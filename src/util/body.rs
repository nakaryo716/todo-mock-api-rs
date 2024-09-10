use http_body_util::BodyExt;
use hyper::{body::Incoming, Request};
use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum DeserializeError {
    #[error("collect body error")]
    CollectBodyFail,
    #[error("parse body error")]
    ParseError,
    #[error("encode body error")]
    NotUtf8,
}

pub async fn deserialize_body<'a, T>(req: Request<Incoming>) -> Result<T, DeserializeError>
where
    T: DeserializeOwned + Send,
{
    let req_body_byte = req
        .collect()
        .await
        .map_err(|_e| DeserializeError::CollectBodyFail)?
        .to_bytes();

    let json_payload =
        String::from_utf8(req_body_byte.to_vec()).map_err(|_e| DeserializeError::NotUtf8)?;

    let deserialized_payload =
        serde_json::from_str::<T>(&json_payload).map_err(|_e| DeserializeError::ParseError)?;

    Ok(deserialized_payload)
}
