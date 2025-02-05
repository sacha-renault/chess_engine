// Evaluator const values
pub const CASTLING_BONUS: f32 = 5.;
pub const CAPTURE_BONUS: f32 = 1.;
pub const CAPTURE_MVV_LVA_FACTOR: f32 = 1.;
pub const CHECK_BONUS: f32 = 5.;
pub const CHECK_MATE: f32 = 1e5 as f32;
pub const VALUE_TB_WIN_IN_MAX_PLY: f32 = (CHECK_MATE as f32) / 2.;
pub const WHITE_PAWNS_VALUE: [f32; 64] = [
    0.83, 0.83, 0.83, 0.83, 0.83, 0.83, 0.83, 0.83, 0.87, 0.87, 0.87, 0.9, 0.9, 0.87, 0.87, 0.87,
    0.9, 0.91, 0.92, 0.98, 0.98, 0.92, 0.91, 0.9, 0.94, 0.95, 0.96, 1.05, 1.05, 0.96, 0.95, 0.94,
    0.98, 0.99, 1.01, 1.12, 1.12, 1.01, 0.99, 0.98, 1.01, 1.03, 1.05, 1.2, 1.2, 1.05, 1.03, 1.01,
    1.05, 1.07, 1.1, 1.27, 1.27, 1.1, 1.07, 1.05, 1.09, 1.1, 1.14, 1.35, 1.35, 1.14, 1.1, 1.09,
];
pub const BLACK_PAWNS_VALUE: [f32; 64] = [
    1.09, 1.1, 1.14, 1.35, 1.35, 1.14, 1.1, 1.09, 1.05, 1.07, 1.1, 1.27, 1.27, 1.1, 1.07, 1.05,
    1.01, 1.03, 1.05, 1.2, 1.2, 1.05, 1.03, 1.01, 0.98, 0.99, 1.01, 1.12, 1.12, 1.01, 0.99, 0.98,
    0.94, 0.95, 0.96, 1.05, 1.05, 0.96, 0.95, 0.94, 0.9, 0.91, 0.92, 0.98, 0.98, 0.92, 0.91, 0.9,
    0.87, 0.87, 0.87, 0.9, 0.9, 0.87, 0.87, 0.87, 0.83, 0.83, 0.83, 0.83, 0.83, 0.83, 0.83, 0.83,
];
pub const BISHOPS_VALUE: [f32; 64] = [
    0.95, 0.95, 0.83, 0.83, 0.83, 0.83, 0.95, 0.95, 0.95, 1.19, 1.07, 0.95, 0.95, 1.07, 1.19, 0.95,
    0.83, 1.07, 1.19, 1.07, 1.07, 1.19, 1.07, 0.83, 0.83, 0.95, 1.07, 1.3, 1.3, 1.07, 0.95, 0.83,
    0.83, 0.95, 1.07, 1.3, 1.3, 1.07, 0.95, 0.83, 0.83, 1.07, 1.19, 1.07, 1.07, 1.19, 1.07, 0.83,
    0.95, 1.19, 1.07, 0.95, 0.95, 1.07, 1.19, 0.95, 0.95, 0.95, 0.83, 0.83, 0.83, 0.83, 0.95, 0.95,
];
pub const KNIGHTS_VALUE: [f32; 64] = [
    0.93, 0.95, 0.96, 0.97, 0.97, 0.96, 0.95, 0.93, 0.95, 0.97, 0.99, 1.01, 1.01, 0.99, 0.97, 0.95,
    0.96, 0.99, 1.03, 1.07, 1.07, 1.03, 0.99, 0.96, 0.97, 1.01, 1.07, 1.19, 1.19, 1.07, 1.01, 0.97,
    0.97, 1.01, 1.07, 1.19, 1.19, 1.07, 1.01, 0.97, 0.96, 0.99, 1.03, 1.07, 1.07, 1.03, 0.99, 0.96,
    0.95, 0.97, 0.99, 1.01, 1.01, 0.99, 0.97, 0.95, 0.93, 0.95, 0.96, 0.97, 0.97, 0.96, 0.95, 0.93,
];

// Tree const values
pub const RAZORING_MARGIN_BASE: f32 = 50.;
pub const RAZORING_DEPTH: usize = usize::MAX;
pub const RAZORING_DEPTH_MULTIPLIER: f32 = 0.8;
pub const HEURISTIC_WEIGHT: f32 = 5.;
pub const UNCERTAINTY_MALUS: f32 = 50000.;
