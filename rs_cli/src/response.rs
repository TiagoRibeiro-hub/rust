#[derive(Debug)]
pub struct Response {
    pub message: String,
    pub succeed: bool,
}

impl Default for Response {
    fn default() -> Self {
        Response {
            message: String::from("Something went wrong"),
            succeed: false,
        }
    }
}
