// [1] [2] [3]
// [4]  x  [6]
// [7] [8] [9]
// > 5

use std::fmt;
use std::error::Error;

#[derive(Copy, Clone, PartialEq)]
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

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Empty,
    Figure(Player)
}

#[derive(Copy, Clone)]
enum State {
    PlayerTurn(Player),
    GameOver(Option<Player>)
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
            if trimmed_input == "q" {
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
    }}


fn board_index(i: usize, j: usize) -> usize {
    i * 3 + j
}

fn print_board(board: &[Cell; 9]) {
    for (i, row) in board.chunks(3).enumerate() {
        for (j, cell) in row.iter().enumerate() {
            print_cell(cell, board_index(i, j) + 1)
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

fn player_won(board: &[Cell; 9], player: Player) -> bool {
    for i in 0..3 {
        let mut row_streak = true;
        let mut col_streak = true;

        for j in 0..3 {
            let row_index = board_index(i, j);
            let col_index = board_index(j, i);

            row_streak = row_streak && (board[row_index] == Figure(player));
            col_streak = col_streak && (board[col_index] == Figure(player))
        }

        if row_streak {
            return true
        }

        if col_streak {
            return true
        }
    }

    false
}

fn player_turn(board: &mut [Cell; 9],
               player: Player) -> State {
    print_board(board);
    println!("{}> ", player);

    match read_command() {
        Ok(Put(index)) => if 1 <= index && index <= 9 {
            if let Empty = board[index - 1] {
                board[index - 1] = Figure(player);

                if player_won(board, player) {
                    GameOver(Some(player))
                } else {
                    PlayerTurn(opposite_player(player))
                }
            } else {
                println!("The cell is not empty!");
                PlayerTurn(player)
            }
        } else {
            println!("Incorrect index. Please try again");
            PlayerTurn(player)
        },

        _ => GameOver(None)
    }
}

fn main() {
    let mut board = [Empty; 9];
    let mut state = PlayerTurn(Cross);

    while let PlayerTurn(player) = state {
        state = player_turn(&mut board, player)
    }

    match state {
        GameOver(Some(player)) => println!("{} won", player),
        _ => ()
    }
}
