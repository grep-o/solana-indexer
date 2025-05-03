use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct DynamicResponse<T> {
    pub success: bool,
    pub data: Option<T>,
}

#[derive(Serialize)]
pub struct Message {
    pub message: String,
}

impl<T: Serialize> DynamicResponse<T> {
    pub fn success(data: Option<T>) -> Self {
        Self { success: true, data }
    }

    pub fn failure(data: Option<T>) -> Self {
        Self { success: false, data }
    }

    pub fn with_status(self, status: StatusCode) -> (StatusCode, Json<Self>) {
        (status, Json(self))
    }
}

impl<T: Serialize> IntoResponse for DynamicResponse<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl DynamicResponse<Message> {
    pub fn success_message(message: &str, status: Option<StatusCode>) -> Response {
        let message = Message { message: message.to_string() };

        Self::success(Some(message)).with_status(status.unwrap_or(StatusCode::OK)).into_response()
    }

    pub fn failure_message(message: &str, status: Option<StatusCode>) -> Response {
        let message = Message { message: message.to_string() };

        Self::failure(Some(message)).with_status(status.unwrap_or(StatusCode::BAD_REQUEST)).into_response()
    }

    pub fn user_not_found() -> Response {
        Self::failure_message("User not found", Some(StatusCode::UNAUTHORIZED))
    }

    pub fn failure_empty() -> Response {
        Self::failure_message("Something went wrong", None)
    }
}

impl<T: Serialize> DynamicResponse<T> {
    pub fn success_data(data: T) -> Response {
        Self::success(Some(data)).into_response()
    }
}

impl DynamicResponse<()> {
    pub fn success_empty() -> Response {
        Self::success(None).into_response()
    }
}
