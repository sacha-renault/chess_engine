use urlencoding;

use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, ACCEPT};
use serde_json;

use super::models::{LichessMasterDbResponse, LichessMove};
use super::api_error::ApiError;

fn build_request(fen: &str) -> String {
    format!("https://explorer.lichess.ovh/masters?fen={}", urlencoding::encode(fen))
}

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