use colored::Colorize;

pub const BOARD_SIZE: usize = 3;

#[derive(Debug)]
pub struct Board {
    cells: [[Option<char>; BOARD_SIZE]; BOARD_SIZE],
}

pub enum BoardState {
    Ended(Option<Player>),
    Ongoing,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Player {
    O,
    X,
}

impl Player {
    pub fn get_player_char_from_enum(player: &Player) -> char {
        match player {
            Player::O => 'O',
            Player::X => 'X',
        }
    }

    pub fn get_player_enum_from_char(c: char) -> Option<Player> {
        match c {
            'O' => Some(Player::O),
            'X' => Some(Player::X),
            _ => None,
        }
    }
}

pub struct Move {
    pub position: u32,
    pub player: Player,
}

impl Move {
    pub fn new(position: u32, player: Player) -> Result<Self, &'static str> {
        if position > 0 && position <= 9 {
            Ok(Move {
                position: position,
                player: player,
            })
        } else {
            Err("Invalid position. Position should be in the range 0 < position <= 9")
        }
    }
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: [[None; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    pub fn display(&self) {
        let horizontal_borders = "-".repeat(19);

        println!("{}", horizontal_borders);

        for i in 0..self.cells.len() {
            for j in 0..self.cells[0].len() {
                // println!("Row: {:?}, Col: {:?}", row, col);
                let value = match self.cells[i][j] {
                    Some(val) => match val.to_string().to_ascii_lowercase().as_str() {
                        "x" => "X".red(),
                        "o" => "O".blue(),
                        others => others.white(),
                    },
                    None => {
                        let val = ((i * BOARD_SIZE) + j + 1).to_string(); // TODO: MOVE TO ANOTHER FUNCTION
                        val.white()
                    }
                };
                print!("|  {}  ", value);
            }
            println!("|");
            println!("{}", horizontal_borders);
        }
        println!("Total open slots: {}", self.get_number_of_open_slots());
    }

    fn get_number_of_open_slots(&self) -> usize {
        let mut total = 0;

        for row in self.cells {
            for col in row {
                if let None = col {
                    total += 1;
                } 
            }
        }
        total
    }

    fn is_slot_empty(&self, position: usize) -> bool {
        let mut empty = true;
        for (i, row) in self.cells.iter().enumerate() {
            for (j, num) in row.iter().enumerate() {
                if (i * BOARD_SIZE) + j + 1 == position {
                    if let Some(_) = num {
                        empty = false;
                    }
                }
            }
        }
        // slot = self.cells[]
        empty
    }

    fn check_valid_move(&self, player_move: &Move) -> bool {
        let mut valid_move = true;
        if player_move.player != self.get_next_player() {
            valid_move = false;
        }

        if !self.is_slot_empty(player_move.position as usize) {
            valid_move = false;
        }

        valid_move
    }

    pub fn get_next_player(&self) -> Player {
        if (self.get_number_of_open_slots() % 2) == 0 {
            Player::X
        } else {
            Player::O
        }
    }

    fn check_rows(&self) -> Option<Player> {
        for row in &self.cells {
            if let Some(player) = self.check_equal_cells(row.iter()) {
                return Some(player);
            }
        }

        None
    }

    fn check_columns(&self) -> Option<Player> {
        for col in 0..BOARD_SIZE {
            let column = self.cells.iter().map(|row| &row[col]);
            if let Some(player) = self.check_equal_cells(column) {
                return Some(player);
            }
        }
        None
    }

    fn check_diagonals(&self) -> Option<Player> {
        let primary_diagonal = (0..BOARD_SIZE).map(|i| &self.cells[i][i]);
        let secondary_diagonal = (0..BOARD_SIZE).map(|i| &self.cells[i][BOARD_SIZE - 1 - i]);

        if let Some(player) = self.check_equal_cells(primary_diagonal) {
            return Some(player);
        }

        if let Some(player) = self.check_equal_cells(secondary_diagonal) {
            return Some(player);
        }

        None
    }

    fn check_equal_cells<'a, I>(&self, cells: I) -> Option<Player>
    where
        I: Iterator<Item = &'a Option<char>>,
    {
        let values: Vec<&Option<char>> = cells.collect();
        let first_value = values[0];

        if values.iter().all(|&v| v == first_value && v.is_some()) {
            return Player::get_player_enum_from_char(first_value.unwrap());
        }

        None
    }

    fn game_winner(&self) -> Option<Player> {
        // Check rows
        if let Some(player) = self.check_rows() {
            return Some(player);
        }

        if let Some(player) = self.check_columns() {
            return Some(player);
        }

        if let Some(player) = self.check_diagonals() {
            return Some(player);
        }

        None
    }

    pub fn make_move(&mut self, player_move: Move) -> Result<BoardState, &'static str> {
        // Check that game has not ended.
        if let Some(_) = self.game_winner() {
            return Err("Game has already ended");
        }
        // Check valid move
        if self.check_valid_move(&player_move) {
            // Make the move

            for (i, row) in self.cells.iter_mut().enumerate() {
                for (j, num) in row.iter_mut().enumerate() {
                    if (i * BOARD_SIZE) + j + 1 == player_move.position as usize {
                        if let None = num {
                            *num = Some(Player::get_player_char_from_enum(&player_move.player));
                        } else {
                            return Err("Invalid Move");
                        }
                    }
                }
            }
            // Check game ended
            if let Some(winner) = self.game_winner() {
                return Ok(BoardState::Ended(Some(winner)));
            } else {
                if self.get_number_of_open_slots() == 0 {
                    println!("Open slots: {}", self.get_number_of_open_slots());
                    return Ok(BoardState::Ended(None));
                }
                return Ok(BoardState::Ongoing);
            }
        } else {
            return Err("Invalid move.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board_initialization() {
        let board = Board::new();

        // Check Board is correct size
        assert_eq!(board.cells.len(), BOARD_SIZE);
        assert_eq!(board.cells[0].len(), BOARD_SIZE)
    }

    #[test]
    fn test_slot_empty() {
        let board = Board::new();

        for position in 1..=BOARD_SIZE.pow(2) {
            assert!(
                board.is_slot_empty(position),
                "Position {} was not empty",
                position
            );
        }
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_number_of_open_slots() {
        let mut board = Board::new();

        assert_eq!(board.get_number_of_open_slots(), BOARD_SIZE.pow(2));

        let player_move = Move {
            position: 1,
            player: Player::O,
        };
        board.make_move(player_move);
        assert_eq!(board.get_number_of_open_slots(), BOARD_SIZE.pow(2) - 1, "Open Slots {} != {}", board.get_number_of_open_slots(), BOARD_SIZE.pow(2) - 1);
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_check_invalid_move() {
        let mut board = Board::new();

        const POSITION: u32 = 1;
        const PLAYER: Player = Player::O;
        let player_move = Move {
            position: POSITION,
            player: PLAYER,
        };
        board.make_move(player_move);

        let player_move = Move {
            position: POSITION,
            player: PLAYER,
        };

        assert!(!board.check_valid_move(&player_move));
    }
}
