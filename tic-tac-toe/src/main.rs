use colored::*;
use std::io;
mod brains;
use brains::{TicTacToeBrain, BrainLevelOne};
mod tic_tac_toe;
use tic_tac_toe::{Board, BoardState, Move, Player, BOARD_SIZE};
use rand::Rng;
enum GameMode {
    AgainstComputer(Player, Box<dyn TicTacToeBrain>),
    AgainstHuman,
}

fn print_error(error: &str) {
    eprintln!("{}", error.red());
}

fn ask_for_game_mode() -> GameMode {
    let result = loop {
        println!("Choose game mode");
        println!("1. Play Against Computer");
        println!("2. Play Against Human");

        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("An error occurred while reading your string");

        let player_variants = [Player::O, Player::X];
        let index = rand::thread_rng().gen_range(0..player_variants.len());
        let chosen_player = player_variants[index];

        match buf.trim() {
            "1" => break GameMode::AgainstComputer(chosen_player, Box::new(BrainLevelOne)),
            "2" => break GameMode::AgainstHuman,
            _ => print_error("Invalid option selected"),
        }
    };

    result
}

fn ask_for_starting_player() -> Player {
    let player = loop {
        println!(
            "Select first player, {} or {}",
            Player::get_player_char_from_enum(&Player::X).to_ascii_uppercase(),
            Player::get_player_char_from_enum(&Player::O).to_ascii_uppercase()
        );

        let mut player_buffer = String::new();
        io::stdin()
            .read_line(&mut player_buffer)
            .expect("An error occurred while reading your string");

        match Player::get_player_enum_from_char(
            player_buffer.chars().next().expect("Invalid input"),
        ) {
            Some(player) => {
                break player;
            }
            None => {
                print_error("Please choose either X or O");
                continue;
            }
        };
    };

    player
}

fn ask_for_move_position(player: &Player) -> Result<usize, &'static str> {
    println!(
        "{}: Select your move ({} a number between {} and {} then press enter)",
        Player::get_player_char_from_enum(&player),
        "press".blue(),
        "1".blue(),
        "9".blue()
    );

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("An error occured while reading your string");

    let position: usize = match buffer.trim().parse() {
        Ok(position) => position,
        Err(_) => return Err("Invalid Position Selected"),
    };

    Ok(position)
}



fn main() {
    println!("Tic Tac Toe game");
    // 1. Ask for Computer vs Player
    //      - If Computer:
    //          1. Level 1 or Level2
    //
    // 2. Ask for Starting Player


    let player_1 = ask_for_starting_player();
    let game_mode = ask_for_game_mode();
    let mut board = Board::new(player_1);
    let mut game_ended = false;


    match &game_mode {
        GameMode::AgainstComputer(computer_player, _) => {
            if player_1 == *computer_player {
                println!("Computer has chosen: {}", Player::get_player_char_from_enum(computer_player));
            }
        }
        GameMode::AgainstHuman => {

        }
    }

    while !game_ended {
        println!("Current board: ");
        board.display();

        let player = board.get_next_player();

        let player_move:Move;
        let position: usize;
        match &game_mode {
            GameMode::AgainstComputer(computer_player, brain) => {
                if player == *computer_player {
                    player_move = match brain.make_move(&board) {
                        Ok(player_move) => player_move,
                        Err(err) => {
                            print_error(&err);
                            continue;
                        }
                    };
                } else {
                    position = match ask_for_move_position(&player) {
                        Ok(num) => {
                            if num > 0 && num <= BOARD_SIZE.pow(2) as usize {
                                num
                            } else {
                                {
                                    print_error("Invalid Position selected");
                                    continue;
                                }
                            }
                        }
                        Err(err) => {
                            print_error(err);
                            continue;
                        }
                    };
    
                    player_move = match Move::create(position, player) {
                        Ok(player_move) => player_move,
                        Err(err) => {
                            print_error(err.as_str());
                            continue;
                        }
                    };
                }
                
            }
            GameMode::AgainstHuman => {
                position = match ask_for_move_position(&player) {
                    Ok(num) => {
                        if num > 0 && num <= BOARD_SIZE.pow(2) as usize {
                            num
                        } else {
                            {
                                print_error("Invalid Position selected");
                                continue;
                            }
                        }
                    }
                    Err(err) => {
                        print_error(err);
                        continue;
                    }
                };

                player_move = match Move::create(position, player) {
                    Ok(player_move) => player_move,
                    Err(err) => {
                        print_error(err.as_str());
                        continue;
                    }
                };
            }
        }
        


        match board.make_move(player_move) {
            Err(msg) => {
                print_error(format!("An error occured while making your move.\n{}", msg).as_str());
            }
            Ok(state) => {
                board.display();
                match state {
                    BoardState::Ended(player) => {
                        match player {
                            Some(winner) => {
                                println!(
                                    "{} {} {}",
                                    "Player".green(),
                                    Player::get_player_char_from_enum(&winner)
                                        .to_string()
                                        .green(),
                                    "won the game!!".green()
                                );
                                board.display();
                            }
                            None => println!("Tie Game!"),
                        }
                        println!("------------------------------------------------------------");

                        game_ended = true;
                    }
                    _ => {}
                }
            }
        }

        println!("\n");
    }
}
