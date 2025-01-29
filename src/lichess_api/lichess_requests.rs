use urlencoding;

use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, ACCEPT};
use serde_json;

use super::models::{LichessMasterDbResponse, LichessMove};
use super::api_error::ApiError;

fn build_request(fen: &str) -> String {
    format!("https://explorer.lichess.ovh/masters?fen={}", urlencoding::encode(fen))
}

/// Fetches move statistics from the Lichess Masters database for a given position.
///
/// This function makes an authenticated HTTP request to the Lichess API to retrieve
/// statistics about moves played in master-level games from a specific position.
///
/// # Arguments
///
/// * `fen` - A string slice containing the FEN (Forsyth–Edwards Notation) representation of the chess position
/// * `api_key` - A string slice containing the Lichess API authentication token
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok(Vec<LichessMove>)` - A vector of move statistics from the position
/// * `Err(ApiError)` - An error that occurred during the API request, which can be:
///   * `ApiError::HttpError` - Failed to make the HTTP request
///   * `ApiError::OpenContentError` - Failed to read the response content
///   * `ApiError::JsonError` - Failed to parse the JSON response
pub fn fetch_lichess_moves(fen: &str, api_key: &str) -> Result<Vec<LichessMove>, ApiError> {
    let client = Client::new();
    let url = build_request(&fen);
    let response = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .header(ACCEPT, "application/json")
        .send()
        .map_err(|err| ApiError::HttpError(err.to_string()))?;
    let content = response
        .text()
        .map_err(|_| ApiError::OpenContentError)?;
    let data: LichessMasterDbResponse = serde_json::from_str(&content)
        .map_err(|err| ApiError::JsonError(err.to_string()))?;

    Ok(data.moves)
}