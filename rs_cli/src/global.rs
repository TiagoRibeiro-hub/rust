#[derive(thiserror::Error, Debug)]
pub enum CustomError {
    #[error("{0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}

#[derive(Debug)]
pub struct Response {
    pub message: String,
    pub succeed: bool,
}