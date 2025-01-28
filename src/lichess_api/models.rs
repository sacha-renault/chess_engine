use serde::Deserialize;

#[derive(Deserialize)]
pub struct LichessMove {
    pub san: String,
    pub white: u32,
    pub draws: u32,
    pub black: u32,
    pub average_rating: u32
}

#[derive(Deserialize)]
pub struct LichessMasterDbResponse {
    pub moves: Vec<LichessMove>
}