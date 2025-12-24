use axum::response::IntoResponse;
use bytes::{BufMut, Bytes, BytesMut};
use http::{
    StatusCode,
    header::{self, HeaderMap, HeaderValue},
};
use sea_orm::DbErr;
use snafu::prelude::*;

pub type Result<T> = std::result::Result<T, ErrorV2>;

pub enum Resp<T> {
    Data(T),
    Err(ErrorV2),
}

impl<T> From<Result<T>> for Resp<T> {
    fn from(value: Result<T>) -> Self {
        match value {
            Ok(d) => Resp::Data(d),
            Err(err) =>{
                println!("Resp err:{}", err);
                 Resp::Err(err)
            },
        }
    }
}

#[derive(Clone, serde::Serialize)]
struct InnerResp<T: serde::Serialize> {
    code: String,
    msg: String,
    data: Option<T>,
}

impl<T: serde::Serialize> IntoResponse for Resp<T> {
    fn into_response(self) -> axum::response::Response {
        let mut buf = bytes::BytesMut::with_capacity(128).writer();

        let inner_resp = match self {
            Resp::Data(d) => InnerResp::<T> {
                code: String::from("0"),
                msg: String::from("success"),
                data: Option::Some(d),
            },
            Resp::Err(error) => InnerResp::<T> {
                code: String::from("1"),
                msg: String::from("1"),
                data: Option::None,
            },
        };

        return match serde_json::to_writer(&mut buf, &inner_resp) {
            Ok(_) => (
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
                )],
                buf.into_inner().freeze(),
            )
                .into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                )],
                err.to_string(),
            )
                .into_response(),
        };
    }
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("db exec failed msg {}", msg))]
    DBError { source: sea_orm::DbErr, msg: String },

    #[snafu(display("db exec failed msg {}", msg))]
    VersionConflict { msg: String },

    #[snafu(whatever, display("db exec failed msg {}", message))]
    ParamError { message: String },
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorV2 {
    // #[error("data store disconnected")]
    // Disconnect(#[from] io::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader {
    //     expected: String,
    //     found: String,
    // },
    // #[error("unknown data store error")]
    // Unknown,
    #[error("param error {0}")]
    ParamError(String),

    #[error("db error (msg: {msg}, error: {source})")]
    DBError {
        #[source]
        source: sea_orm::DbErr,
        msg: String,
    },
}

pub trait ResultExt<T, E>: Sized {}
