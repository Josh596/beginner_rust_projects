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
                        "x" => "x".to_ascii_uppercase().cyan(),
                        "o" => "o".to_ascii_uppercase().yellow(),
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


    fn game_winner(&self) -> Option<Player> {
        let mut winner: Option<Player> = None;

        // Check rows
        for row in self.cells {
            if row.iter().all(|&x| x == row[0]) {
                if let Some(winner_char) = row[0] {
                    winner = Player::get_player_enum_from_char(winner_char);
                }
            }
        }

        // Check cols
        for j in 0..self.cells.len() {
            let mut equal = true;
            let value = self.cells[0][j];
            for i in 0..self.cells.len() {
                if value != self.cells[i][j] {
                    equal = false;
                    break;
                }
            }

            if equal {
                if let Some(winner_char) = value {
                    winner = Player::get_player_enum_from_char(winner_char);
                }
            }
        }

        // Check two diagonals
        // Check primary diagonals
        let mut primary_diagonal_equal = true;
        for i in 0..self.cells.len() {
            if self.cells[i][i] != self.cells[0][0] {
                primary_diagonal_equal = false;
                break;
            }
        }
        if primary_diagonal_equal {
            if let Some(c) = self.cells[0][0] {
                winner = Player::get_player_enum_from_char(c);
            }
        }

        // Check secondary diagonal
        let mut secondary_diaginal_equal = true;

        for i in 0..self.cells.len() {
            if self.cells[i][self.cells.len() - 1 - i] != self.cells[0][self.cells.len() - 1] {
                secondary_diaginal_equal = false;
            }
        }

        if secondary_diaginal_equal {
            if let Some(c) = self.cells[0][self.cells.len() - 1] {
                winner = Player::get_player_enum_from_char(c);
            }
        }

        winner
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
