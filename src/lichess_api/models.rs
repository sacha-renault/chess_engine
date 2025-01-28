use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LichessMove {
    pub san: String,
    pub white: u32,
    pub draws: u32,
    pub black: u32,
    #[serde(rename = "averageRating")]
    pub average_rating: u32
}

#[derive(Debug, Deserialize)]
pub struct LichessMasterDbResponse {
    pub moves: Vec<LichessMove>
}