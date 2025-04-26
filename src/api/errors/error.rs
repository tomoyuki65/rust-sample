use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommonError {
    #[error("Internal Server Error")]
    InternalServerError,
}
