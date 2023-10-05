#[derive(Debug, Default)]
pub struct Response {
    pub message: String,
    pub succeed: bool,
}

pub trait Default {
    fn default() -> Response {
        Response {
            message: String::from("Something went wrong"),
            succeed: false,
        }
    }
}
