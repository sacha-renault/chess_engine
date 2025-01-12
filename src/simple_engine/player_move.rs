use super::utility::coordinates_to_u64;

pub enum Castling {
    Long,
    Short,
}

pub struct NormalMove {
    current_square: u64,
    target_square: u64,
}

impl NormalMove {
    pub fn new(current_square: u64, target_square: u64) -> Self {
        NormalMove {
            current_square,
            target_square,
        }
    }

    pub fn new_from_coordinates(current: (usize, usize), target: (usize, usize)) -> Self {
        // Get coordinates as square
        let current_square = coordinates_to_u64(current);
        let target_square = coordinates_to_u64(target);
        Self::new(current_square, target_square)
    }

    pub fn squares(&self) -> (u64, u64) {
        (self.current_square, self.target_square)
    }
}

pub enum PlayerMove {
    NormalMove(NormalMove),
    Castling(Castling),
}
