// tic_tac_toe.rs

/// Represents the Tic Tac Toe game board.
/// The board consists of a 3x3 grid of cells, each of which can hold an `Option<char>`
/// representing either an 'X' or an 'O' player's move.
use colored::Colorize;

/// Constant size of the tic tac toe board.
/// Tic Tac Toe boards are 3x3, and I'm unsure if this module would work for a 4x4 board.
pub const BOARD_SIZE: usize = 3;

/// Represents the Tic Tac Toe game board.
/// The board consists of a 3x3 grid of cells, each of which can hold an `Option<char>`
/// representing either an 'X' or an 'O' player's move.
#[derive(Debug)]
pub struct Board {
    cells: [[Option<char>; BOARD_SIZE]; BOARD_SIZE],
    player_1: Player,
}

/// Represents the possible states of the Tic Tac Toe game.
/// The game can be in an ongoing state or can have ended with a winner or a tie.
pub enum BoardState {
    /// The game has ended. If `Some(Player)`, the indicated player has won.
    /// If `None`, the game ended in a tie.
    Ended(Option<Player>),
    /// The game is still ongoing.
    Ongoing,
}
/// Represents a player in the Tic Tac Toe game.
/// Players can be 'X' or 'O'.
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Player {
    O,
    X,
}

impl Player {
    /// Returns the character representation of the player.
    pub fn get_player_char_from_enum(player: &Player) -> char {
        match player {
            Player::O => 'O',
            Player::X => 'X',
        }
    }
    /// Converts a character to a player enum.
    /// Returns `Some(Player)` for 'X' or 'O', and `None` for other characters.
    pub fn get_player_enum_from_char(c: char) -> Option<Player> {
        match c.to_ascii_uppercase() {
            'O' => Some(Player::O),
            'X' => Some(Player::X),
            _ => None,
        }
    }
}
/// Represents a move made by a player.
pub struct Move {
    /// The position on the board where the move is made (1 to 9).
    position: usize,
    /// The player making the move.
    player: Player,
}

impl Move {
    /// Creates a new move instance.
    /// Returns an error if the position is outside the valid range, i.e 1-9.
    pub fn create(position: usize, player: Player) -> Result<Self, String> {
        if position > 0 && position <= 9 {
            Ok(Move {
                position: position,
                player: player,
            })
        } else {
            let error_message = format!("Invalid position {}. Position should be in the range 0 < position <= 9", position);
            Err(error_message)
        }
    }
}

impl Board {
    /// Creates a new instance of the Tic Tac Toe game board.
    pub fn new(player_1:Player) -> Self {
        Board {
            cells: [[None; BOARD_SIZE]; BOARD_SIZE],
            player_1: player_1
        }
    }

    /// Displays the current state of the board.
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

    /// Returns the total number of open slots on the board.
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

    // Checks if a particular slot is empty or occupied.
    ///
    /// This function checks whether the specified position on the game board is empty or contains a symbol.
    ///
    /// # Parameters
    ///
    /// - `position`: The position to check (1-based index).
    ///
    /// # Returns
    ///
    /// - `true` if the specified slot is empty.
    /// - `false` if the specified slot is occupied.
    pub fn is_slot_empty(&self, position: usize) -> bool {
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

    /// Checks if a move is valid.
    ///
    /// This function verifies whether a player's move is valid based on the following criteria:
    /// - The player's move is in sequence (e.g., if it's 'X's turn, the next move should be 'O's).
    /// - The selected slot is not already occupied.
    ///
    /// # Parameters
    ///
    /// - `player_move`: The move to be validated.
    ///
    /// # Returns
    ///
    /// - `true` if the move is valid.
    /// - `false` if the move is invalid. 
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

    /// Returns the next player whose turn it is.
    ///
    /// The next player is determined based on the number of open slots on the board.
    /// If the number of open slots is even, it's Player 'X'; otherwise, it's Player 'O'.
    ///
    /// # Returns
    ///
    /// - `Player::X` if the number of open slots is even.
    /// - `Player::O` if the number of open slots is odd.
    pub fn get_next_player(&self) -> Player {
        let player_1: Player;
        let player_2: Player;
        match self.player_1 {
            Player::O => {
                player_1 = Player::O;
                player_2 = Player::X;
            }
            Player::X => {
                player_1 = Player::X;
                player_2 = Player::O;
            }
        }
        if (self.get_number_of_open_slots() % 2) == 0 {
            player_2
        } else {
            player_1
        }
    }

    /// Checks rows of the game board for a winner.
    ///
    /// This function iterates through each row of the board and calls `check_equal_cells`
    /// to determine if all cells in a row have the same non-empty value (either 'X' or 'O').
    ///
    /// # Returns
    ///
    /// - `Some(Player)` if a winning player is found.
    /// - `None` if no winner is found.
    fn check_rows(&self) -> Option<Player> {
        for row in &self.cells {
            if let Some(player) = self.check_equal_cells(row.iter()) {
                return Some(player);
            }
        }

        None
    }

    /// Checks columns of the game board for a winner.
    ///
    /// This function iterates through each column of the board and calls `check_equal_cells`
    /// to determine if all cells in a column have the same non-empty value (either 'X' or 'O').
    ///
    /// # Returns
    ///
    /// - `Some(Player)` if a winning player is found.
    /// - `None` if no winner is found.
    fn check_columns(&self) -> Option<Player> {
        for col in 0..BOARD_SIZE {
            let column = self.cells.iter().map(|row| &row[col]);
            if let Some(player) = self.check_equal_cells(column) {
                return Some(player);
            }
        }
        None
    }

    /// Checks diagonals of the game board for a winner.
    ///
    /// This function checks both the primary and secondary diagonals of the board
    /// by calling `check_equal_cells` with the appropriate iterators.
    ///
    /// # Returns
    ///
    /// - `Some(Player)` if a winning player is found.
    /// - `None` if no winner is found.    
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

    /// Checks if a sequence of cells contains equal non-empty values.
    ///
    /// This function takes an iterator of cell references and checks if all cells
    /// have the same non-empty value (either 'X' or 'O').
    ///
    /// # Parameters
    ///
    /// - `cells`: An iterator over cell references.
    ///
    /// # Returns
    ///
    /// - `Some(Player)` if all cells have the same non-empty value.
    /// - `None` if not all cells have the same value or if any cell is empty.
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

    /// Determines the winner of the game.
    ///
    /// This function checks for a winner by calling `check_rows`.
    ///
    /// # Returns
    ///
    /// - `Some(Player)` if a winning player is found.
    /// - `None` if no winner is found.
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

    /// Makes a move on the game board.
    ///
    /// This function validates the move using `game_winner` and `check_valid_move` functions.
    /// If the move is valid, it updates the board cells and checks if the game has ended.
    ///
    /// # Parameters
    ///
    /// - `player_move`: The move to be made.
    ///
    /// # Returns
    ///
    /// - `Ok(BoardState::Ended(Some(player)))` if the game is won by a player.
    /// - `Ok(BoardState::Ended(None))` if the game ends in a tie.
    /// - `Ok(BoardState::Ongoing)` if the game continues after the move.
    /// - `Err("Game has already ended")` if the game has already been won or tied.
    /// - `Err("Invalid move.")` if the move is not valid.
    pub fn make_move(&mut self, player_move: Move) -> Result<BoardState, String> {
        // Check that game has not ended.
        if self.game_winner().is_some() {
            return Err("Game has already ended".to_string());
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
                            let error_message = format!("Invalid Move: {:}", player_move.position);
                            return Err(error_message);
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
            return Err("Invalid move.".to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board_initialization() {
        let player_1 = Player::X;
        let board = Board::new(player_1);

        // Check Board is correct size
        assert_eq!(board.cells.len(), BOARD_SIZE);
        assert_eq!(board.cells[0].len(), BOARD_SIZE)
    }

    #[test]
    fn test_slot_empty() {
        let player_1 = Player::X;
        let board = Board::new(player_1);

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
        let player_1 = Player::X;
        let mut board = Board::new(player_1);

        assert_eq!(board.get_number_of_open_slots(), BOARD_SIZE.pow(2));

        let player_move = Move {
            position: 1,
            player: board.get_next_player(),
        };
        board.make_move(player_move);
        assert_eq!(
            board.get_number_of_open_slots(),
            BOARD_SIZE.pow(2) - 1,
            "Open Slots {} != {}",
            board.get_number_of_open_slots(),
            BOARD_SIZE.pow(2) - 1
        );
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_check_invalid_move() {
        let player_1 = Player::X;
        let mut board = Board::new(player_1);

        const POSITION: usize = 1;
        let player_2 = board.get_next_player();
        let player_move = Move {
            position: POSITION,
            player: player_2,
        };
        board.make_move(player_move);

        let player_move = Move {
            position: POSITION,
            player: player_2,
        };

        assert!(!board.check_valid_move(&player_move));
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_get_next_player() {
        let player_1 = Player::X;
        let mut board = Board::new(player_1);

        const POSITION: usize = 1;
        let player_2 = board.get_next_player();
        let player_move = Move {
            position: POSITION,
            player: player_2,
        };
        board.make_move(player_move);

        let next_player = board.get_next_player();

        assert_ne!(next_player, player_1);
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_check_rows() {
        let player_1 = Player::X;
        let mut board = Board::new(player_1);

        for i in 1..=BOARD_SIZE {
            let player_o_move = Move {
                position: i as usize,
                player: Player::O,
            };
            board.make_move(player_o_move);
            let player_x_move = Move {
                position: (i + BOARD_SIZE) as usize,
                player: Player::X,
            };
            board.make_move(player_x_move);
        }
        board.display();
        assert!(board.check_rows().is_some())
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_check_col() {
        let player_1 = Player::X;
        let mut board = Board::new(player_1);

        for i in 1..=BOARD_SIZE {
            let player_o_move = Move {
                position: (BOARD_SIZE * i - BOARD_SIZE + 1) as usize,
                player: Player::O,
            };
            board.make_move(player_o_move);
            let player_x_move = Move {
                position: (BOARD_SIZE * i - BOARD_SIZE + 2) as usize,
                player: Player::X,
            };
            board.make_move(player_x_move);
        }
        board.display();
        assert!(board.check_columns().is_some())
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_check_primary_diagonals() {
        // Check Primary Diagon
        let player_1 = Player::O;
        let mut board = Board::new(player_1);

        for i in 1..=BOARD_SIZE {
            let player_o_move = Move {
                position: (BOARD_SIZE * i - (BOARD_SIZE - i)) as usize,
                player: Player::O,
            };
            board.make_move(player_o_move);
            let player_x_move = Move {
                position: (i + 1) as usize,
                player: Player::X,
            };
            board.make_move(player_x_move);
        }
        board.display();
        assert!(board.check_diagonals().is_some())
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_check_secondary_diagonals() {
        // Check Primary Diagon
        let player_1 = Player::O;
        let mut board = Board::new(player_1);

        for i in 1..=BOARD_SIZE {
            let player_o_move = Move {
                position: (BOARD_SIZE * i - (i - 1)) as usize,
                player: Player::O,
            };
            board.make_move(player_o_move);
            let player_x_move = Move {
                position: (i.pow(2)) as usize,
                player: Player::X,
            };
            board.make_move(player_x_move);
        }
        board.display();
        assert!(board.check_diagonals().is_some())
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_game_winner() {
        // Check Primary Diagon
        let player_1 = Player::O;
        let mut board = Board::new(player_1);

        for i in 1..=BOARD_SIZE {
            let player_o_move = Move {
                position: (BOARD_SIZE * i - (i - 1)) as usize,
                player: Player::O,
            };
            board.make_move(player_o_move);
            let player_x_move = Move {
                position: (i.pow(2)) as usize,
                player: Player::X,
            };
            board.make_move(player_x_move);
        }
        board.display();
        assert_eq!(board.game_winner(), Some(Player::O));
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_game_tied() {
        let player_1 = Player::O;
        let mut board = Board::new(player_1);

        // Make moves to fill the entire board without any player winning
        let moves: Vec<Move> = vec![
            Move::create(1, Player::O).unwrap(),
            Move::create(2, Player::X).unwrap(),
            Move::create(3, Player::O).unwrap(),
            Move::create(4, Player::X).unwrap(),
            Move::create(6, Player::O).unwrap(),
            Move::create(5, Player::X).unwrap(),
            Move::create(8, Player::O).unwrap(),
            Move::create(9, Player::X).unwrap(),
            Move::create(7, Player::O).unwrap(),
        ];

        for player_move in moves {
            board.make_move(player_move).unwrap();
        }

        board.display();
        // Check that the game is recognized as a tie
        assert_eq!(board.game_winner(), None);
    }
}
