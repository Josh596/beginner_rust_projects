// Here, I would like to build my AI. One AI that picks positions randomly, and another that uses an algorithm from CS50\
use crate::tic_tac_toe::{Board, Move, BOARD_SIZE};
use rand::Rng;
pub trait TicTacToeBrain {
    fn make_move(&self, board: &Board) -> Result<Move, String>;
}

pub struct BrainLevelOne;
// pub struct BrainLevelTwo;

impl TicTacToeBrain for BrainLevelOne {
    fn make_move(&self, board: &Board) -> Result<Move, String> {
        // Get all empty positions
        // Pick randomly from said positions
        // Make a move.
        let mut empty_positions: Vec<usize> = vec![];

        for position in 1..=BOARD_SIZE.pow(2) {
            if board.is_slot_empty(position) {
                empty_positions.push(position);
            }
        }

        if empty_positions.len() < 1 {
            return Err("No available positions for AI to play. ".to_string());
        }
        let position = rand::thread_rng().gen_range(0..empty_positions.len());
        let chosen_position = empty_positions[position];
        // Should I give the Brain it's own Player when initialized or
        // use Board::get_next_player()
        let player_move = match Move::create(chosen_position, board.get_next_player()) {
            Ok(player_move) => {
                Ok(player_move)
            }
            Err(err) => {
                return Err(err);
            }
        };

        return player_move
    }
}
