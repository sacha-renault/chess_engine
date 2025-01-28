#[derive(Debug)]
pub enum ApiError {
    HttpError(String), // Error from reqwest
    OpenContentError,  // Error from reqwest
    JsonError(String), // Error from serde_json
}