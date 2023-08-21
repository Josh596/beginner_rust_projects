use colored::*;
use std::io;
mod tic_tac_toe;

use tic_tac_toe::{Board, BoardState, Move, Player, BOARD_SIZE};

fn main() {
    let mut board = Board::new();

    let mut game_ended = false;
    println!("Tic Tac Toe game");

    while !game_ended {
        println!("Current board: ");
        board.display();

        let player = board.get_next_player();

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

        let position: u32 = match buffer.trim().parse() {
            Ok(num) => {
                if num > 0 && num <= BOARD_SIZE.pow(2) as u32 {
                    num
                } else {
                    {
                        eprintln!("{}", "Invalid Position selected".red());
                        continue;
                    }
                }
            }
            Err(_) => {
                eprintln!("{}", "Invalid Position selected".red());
                continue;
            }
        };

        let player_move = match Move::new(position, player) {
            Ok(player_move) => player_move,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        match board.make_move(player_move) {
            Err(msg) => {
                eprintln!(
                    "{} \n {}",
                    "An error occured while making your move.".red(),
                    msg.red()
                );
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
                // Check game ended
                // Continu
            }
        }

        println!("");
    }
}
