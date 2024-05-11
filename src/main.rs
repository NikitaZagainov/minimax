mod minimax;
mod tic_tac_toe;
use minimax::best_action;
use std::{io::stdin, process};
use tic_tac_toe::{Action, Mark, TicTacToe};

pub fn check_terminal(game: &TicTacToe) -> bool {
    if game.is_terminal() {
        match game.get_winner() {
            Option::Some(winner) => {
                println!("Game over! Winner: {:?}", winner);
                return true;
            }
            Option::None => {
                println!("Game over! Draw.");
                return true;
            }
        }
    }
    return false;
}

pub fn parse_input() -> Result<(usize, usize), &'static str> {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let s: Vec<&str> = s.strip_suffix("\n").unwrap().split(" ").collect();

    if s.len() != 2 {
        return Err("Error parsing arguments");
    }

    let x = match String::from(s[0])
        .parse::<usize>() {
            Ok(value) => value,
            Err(_) => return Err("Error parsing arguments")
        };

    let y = match String::from(s[1])
        .parse::<usize>() {
            Ok(value) => value,
            Err(_) => return Err("Error parsing arguments")
        };

    return Ok((x, y));
}

fn main() {
    let mut game = TicTacToe::new(4, 'x', 'o', '.', 3);
    println!("=== Tic tac toe ===");
    game.print_board();
    loop {
        let (x, y) = match parse_input() {
            Ok(values) => values,
            Err(msg) => {
                eprintln!("{msg}");
                process::exit(1);
            } 
        };
        game.make_move(&Action::new(x, y, Mark::Player))
            .unwrap_or_else(|err| {
                eprintln!("{err}");
                process::exit(1);
            });
        println!("=== Your move: ===");
        game.print_board();
        if check_terminal(&game) {
            break;
        }

        let bot_action = best_action(5, game.clone());
        game.make_move(&bot_action).unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });
        println!("=== Bot's move: ===");
        game.print_board();
        if check_terminal(&game) {
            break;
        }
    }
}
