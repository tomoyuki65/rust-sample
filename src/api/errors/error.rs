use thiserror::Error;

// OpenAPI用
use utoipa::ToSchema;

#[derive(Error, Debug)]
pub enum CommonError {
    #[error("Internal Server Error")]
    InternalServerError,
}

// OpenAPI用の定義
#[derive(ToSchema)]
pub struct InternalServerErrorResponseBody {
    #[allow(dead_code)]
    #[schema(example = "Internal Server Error")]
    message: String,
}