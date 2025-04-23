use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Forbidden(String),
    UnprocessableEntity(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(message) => (StatusCode::NOT_FOUND, message),
            AppError::Forbidden(message) => (StatusCode::FORBIDDEN, message),
            AppError::UnprocessableEntity(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
        };

        (status, message).into_response()
    }
}
