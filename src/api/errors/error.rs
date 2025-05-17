use axum::http::StatusCode;
use thiserror::Error;

// OpenAPI用
use utoipa::ToSchema;

#[derive(Error, Debug)]
pub enum CommonError {
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("{message}")]
    CustomError {
        status_code: StatusCode,
        message: String,
    },
}

// OpenAPI用の定義
#[derive(ToSchema)]
pub struct InternalServerErrorResponseBody {
    #[allow(dead_code)]
    #[schema(example = "Internal Server Error")]
    message: String,
}

#[derive(ToSchema)]
pub struct CustomErrorResponseBody {
    #[allow(dead_code)]
    #[schema(example = "エラーメッセージ")]
    message: String,
}
