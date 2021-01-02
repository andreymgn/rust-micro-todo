use std::convert::Infallible;

use serde_derive::Serialize;
use tonic::{Code, Status};
use warp::http::StatusCode;
use warp::{Rejection, Reply};

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug)]
pub enum Error {
    RPCError(tonic::Status),
}

impl warp::reject::Reject for Error {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let status;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        status = "not found".to_string();
        message = "not found".to_string();
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        status = "invalid body".to_string();
        message = "invalid body".to_string();
    } else if let Some(e) = err.find::<Error>() {
        match e {
            Error::RPCError(st) => {
                code = map_status_code(&st);
                status = "rpc error".to_string();
                message = st.to_string();
            }
        }
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        status = "method not allowed".to_string();
        message = "method not allowed".to_string();
    } else {
        eprintln!("unhandled error: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        status = "internal server error".to_string();
        message = "internal server error".to_string();
    }

    let json = warp::reply::json(&ErrorResponse { status, message });

    Ok(warp::reply::with_status(json, code))
}

fn map_status_code(st: &Status) -> StatusCode {
    match st.code() {
        Code::Ok => StatusCode::OK,
        Code::Cancelled => StatusCode::INTERNAL_SERVER_ERROR,
        Code::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        Code::InvalidArgument => StatusCode::UNPROCESSABLE_ENTITY,
        Code::DeadlineExceeded => StatusCode::INTERNAL_SERVER_ERROR,
        Code::NotFound => StatusCode::NOT_FOUND,
        Code::AlreadyExists => StatusCode::CONFLICT,
        Code::PermissionDenied => StatusCode::FORBIDDEN,
        Code::ResourceExhausted => StatusCode::INTERNAL_SERVER_ERROR,
        Code::FailedPrecondition => StatusCode::INTERNAL_SERVER_ERROR,
        Code::Aborted => StatusCode::INTERNAL_SERVER_ERROR,
        Code::OutOfRange => StatusCode::INTERNAL_SERVER_ERROR,
        Code::Unimplemented => StatusCode::INTERNAL_SERVER_ERROR,
        Code::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        Code::Unavailable => StatusCode::INTERNAL_SERVER_ERROR,
        Code::DataLoss => StatusCode::INTERNAL_SERVER_ERROR,
        Code::Unauthenticated => StatusCode::UNAUTHORIZED,
        Code::__NonExhaustive => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
