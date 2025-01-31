use crate::game_engine::player_move::PlayerMove;
use crate::tree_search::tree::Tree;
use crate::database::chess_table::ChessTablesDb;
use crate::database::models::MoveModel;
use crate::lichess_api::lichess_requests::fetch_lichess_moves;
use crate::tree_search::tree_node::TreeNodeRef;
use crate::tree_search::tree_trait::SearchEngine;

use super::config::EngineConfig;
use super::next_move::NextMove;

pub struct SmartEngine {
    tree: Tree,
    config: EngineConfig,
    db: ChessTablesDb,
}

impl SmartEngine {
    pub fn new(tree: Tree, config: EngineConfig) -> Result<Self, ()> {
        let db = match &config.db_path {
            Some(db_path) => ChessTablesDb::at_path(db_path.to_path_buf()),
            None => ChessTablesDb::new() 
        }.map_err(|_| ())?;
        Ok(SmartEngine {
            tree,
            config, 
            db
        })
    }

    fn try_get_db_moves(&self, fen: &str) -> Option<Vec<MoveModel>> {
        // First we check if there is known moves
        let moves = self.db.get_moves_by_fen(fen).ok()?;

        // fetch from lichess in same cases
        match (moves.len(), &self.config.lichess_api_key) {
            // Case no result and api_key_provided
            (0, Some(api_key)) => {
                // Get the moves
                let lichess_moves = fetch_lichess_moves(fen, api_key).ok()?;

                // We now convert lichess mvoes to our move type
                // Before inserted (or it would be moved)
                // We use iter not to consume the vec
                let chess_moves = lichess_moves.iter().map(|mv| {
                    // We don't care about the board id it's for inserting in the db
                    MoveModel::from_lichess_move(&mv, 0)
                }).collect::<Vec<_>>();

                // Insert into the db for later usage 
                self.db.insert_board_with_moves(fen, lichess_moves).ok()?;
                
                Some(chess_moves)
            }

            // Case No result but no api key ...
            (0, None) => None,

            // Case where there is already results
            _ => Some(moves),
        }
    }

    fn try_db_search(&mut self) -> Option<NextMove> {
        let fullmove_num = self.tree.root().borrow().get_engine().get_fullmove_number();
        let fen = self.tree.root().borrow().get_engine().to_string();
        let _maximize = self.tree.root().borrow().get_engine().white_to_play();

        // First, if possible, we try can get anything from the database
        let db_moves = self.try_get_db_moves(&fen);
        if fullmove_num <= self.config.max_fullmove_opening && db_moves.is_some() {
            // We can safely unwrap since we checked it wasn't none
            let mut moves = db_moves.unwrap();

            // Sort moves based on evaluation
            moves.sort_by(|a, b| {
                // let cmp = a.to_eval().partial_cmp(&b.to_eval()).unwrap();
                // if maximize { cmp.reverse() } else { cmp }
                b.game_number.partial_cmp(&a.game_number).unwrap()
            });

            // We now return the first result (best move)
            if let Some(best_move) = moves.first() {
                // Convert the san into a player_move
                let chess_move 
                    = self.tree.root().borrow().get_engine().get_move_by_san(&best_move.san).ok()?;

                // Play on the root node 
                // (this will reset any tree if there was one already builded)
                self.tree.root().borrow_mut().play(chess_move.clone()).ok()?;

                return Some(NextMove::new_from_db(
                    chess_move, 
                    best_move.win_rate as f32, 
                    best_move.draw_rate as f32, 
                    best_move.loose_rate as f32));
            }      
        }
        None
    }

    fn try_select_branch(&mut self, chess_move: PlayerMove) -> Result<(), ()>{
        // try select branch
        if self.tree.select_branch(chess_move).is_err() {
            // We couldn't select the branch
            // We try to play on the root directly
            // If it's not working we can't do anything tho
            self.tree
                .root()
                .borrow_mut()
                .play(chess_move.clone())
                .map_err(|_| ())?;
        }
        Ok(())
    }

    fn tree_search(&mut self) -> Option<NextMove> {
        // Start with the iterative deepening that should return the best move
        let best_move = self.tree.search_best_move();

        // ?? Shouldn't happen but we never know
        if best_move.get_move().is_none() {
            return None;
        }

        // Else we can use the chess move
        let chess_move = best_move.get_move().unwrap().clone();
        self.try_select_branch(chess_move).ok()?;

        // Get the mate depth on the new node
        let mate_depth = self.tree.root().borrow().get_plies_to_mate();
        return Some(NextMove::new_from_tree(chess_move, best_move.get_score(), best_move.get_depth(), mate_depth));
    }

    pub fn get_next_move(&mut self) -> Option<NextMove> {
        // Try database moves first
        if let Some(db_move) = self.try_db_search() {
            return Some(db_move);
        }

        // fallback to tree seach
        if let Some(tree_move) = self.tree_search() {
            return Some(tree_move);
        }
        
        // It failed nooooo :'(
        return None;
    }

    pub fn opponent_move(&mut self, chess_move: PlayerMove) -> Result<(), ()> {
        self.try_select_branch(chess_move)
    }

    pub fn opponent_move_san(&mut self, san: &str) -> Result<(), ()> {
        let chess_move =  self.tree
            .root()
            .borrow()
            .get_engine()
            .get_move_by_san(san)
            .map_err(|_| ())?;
        self.try_select_branch(chess_move)
    }

    pub fn white_to_play(&self) -> bool {
        self.tree.root().borrow().get_engine().white_to_play()
    }

    pub fn get_tree_root(&self) -> TreeNodeRef {
        return self.tree.root();
    }

    pub fn tree_size(&self) -> usize {
        return self.tree.size();
    }
}