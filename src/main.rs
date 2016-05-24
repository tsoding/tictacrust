// [1] [2] [3]
// [4]  x  [6]
// [7] [8] [9]
// > 5

use std::fmt;
use std::error::Error;

#[derive(Copy, Clone)]
enum Player {
    Cross,
    Zero
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cross => write!(f, "x"),
            Zero => write!(f, "o")
        }
    }
}

#[derive(Copy, Clone)]
enum Command {
    Quit,
    Put(usize)
}

#[derive(Copy, Clone)]
enum Cell {
    Empty,
    Figure(Player)
}

#[derive(Copy, Clone)]
enum State {
    PlayerTurn(Player),
    GameOver
}

use Player::*;
use Cell::*;
use State::*;
use Command::*;

fn read_command() -> Result<Command, String> {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed_input: &str = input.trim();
            if trimmed_input == "e" {
                Ok(Quit)
            } else {
                Ok(Put(input.trim().parse().unwrap()))
            }
        },
        Err(err) => Err(err.description().to_owned())
    }
}

fn print_cell(cell: &Cell, idx: usize) {
    match *cell {
        Empty => print!("[{}]", idx),
        Figure(player) => print!(" {} ", player)
    }
}

fn print_board(board: &[Cell; 9]) {
    for (i, row) in board.chunks(3).enumerate() {
        for (j, cell) in row.iter().enumerate() {
            print_cell(cell, i * 3 + j + 1)
        }
        println!("")
    }
}

fn opposite_player(player: Player) -> Player {
    match player {
        Cross => Zero,
        Zero => Cross
    }
}

fn player_turn(board: &mut [Cell; 9],
               player: Player) -> State {
    print_board(board);
    println!("{}> ", player);

    match read_command() {
        Ok(Put(index)) => if 1 <= index && index <= 9 {
            if let Empty = board[index - 1] {
                board[index - 1] = Figure(player);
                PlayerTurn(opposite_player(player))
            } else {
                println!("The cell is not empty!");
                PlayerTurn(player)
            }
        } else {
            println!("Incorrect index. Please try again");
            PlayerTurn(player)
        },

        _ => GameOver
    }
}

fn main() {
    let mut board = [Empty; 9];
    let mut state = PlayerTurn(Cross);

    while let PlayerTurn(player) = state {
        state = player_turn(&mut board, player)
    }
}
