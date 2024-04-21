use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::fmt::{Display, Formatter};

pub type Result<T> = core::result::Result<T, Error>;
#[derive(Debug)]
pub enum Error {
    LoginFail,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");
        (StatusCode::UNPROCESSABLE_ENTITY, "Invalid client data").into_response()
    }
}
